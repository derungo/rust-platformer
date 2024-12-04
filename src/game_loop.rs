// game_loop.rs
use crate::engine::{GameState, InputHandler, Renderer};
use crate::engine::renderer::tile::TileMap;
use crate::engine::renderer::instance::InstanceData;

use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};
use pollster::block_on;

pub fn run() {
    // Create an event loop and a window
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title("Rust Platformer Engine")
        .with_inner_size(winit::dpi::PhysicalSize::new(800, 600))
        .build(&event_loop)
        .expect("Failed to create window.");

    // Initialize the renderer
    let mut renderer = block_on(Renderer::new(&window));

    // Initialize the input handler
    let mut input_handler = InputHandler::new();

    // Initialize the game state
    let mut game_state = GameState::new();

    // Create the TileMap
    let tile_map = TileMap::new_ground(
        0.3,
        0.3,
        renderer.tileset_columns,
        renderer.tileset_rows,
    );

    // Timing variables for frame timing
    let mut last_frame_time = std::time::Instant::now();

    // Run the event loop
    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Poll; // Keep the event loop running

        match event {
            // Handle window events
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::CloseRequested => {
                    *control_flow = ControlFlow::Exit; // Exit the application
                }
                WindowEvent::KeyboardInput { input, .. } => {
                    input_handler.handle_keyboard_input(input); // Update input handler
                }
                _ => {}
            },

            // Handle main events cleared
            Event::MainEventsCleared => {
                // Calculate delta time
                let now = std::time::Instant::now();
                let delta_time = now.duration_since(last_frame_time).as_secs_f32();
                last_frame_time = now;

                // Update game state (logic and animations)
                game_state.update(&input_handler, delta_time);

                // Collect instance data
                let mut tile_instances = Vec::new();
                let mut player_instances = Vec::new();

                // Add tiles
                for tile in &tile_map.tiles {
                    // Calculate UV offset and scale
                    let tile_size_u = 1.0 / renderer.tileset_columns as f32;
                    let tile_size_v = 1.0 / renderer.tileset_rows as f32;
                    let u = (tile.tile_index % renderer.tileset_columns) as f32 * tile_size_u;
                    let v = (tile.tile_index / renderer.tileset_columns) as f32 * tile_size_v;
                    let uv_offset = [u, v];
                    let uv_scale = [tile_size_u, tile_size_v];

                    let tile_instance_data = InstanceData {
                        transform: Renderer::create_transform_matrix(
                            tile.position.0,
                            tile.position.1,
                            tile_map.tile_width,
                            tile_map.tile_height,
                        ),
                        sprite_index: 0.0,
                        _padding1: 0.0,
                        sprite_size: [0.0, 0.0], // Use UV logic
                        uv_offset,
                        uv_scale,
                    };
                    tile_instances.push(tile_instance_data);
                }

                // Add player
                let scale_x = if game_state.facing_right { 0.3 } else { -0.3 };
                let sprite_size_x = 1.0 / 24.0; 
                let sprite_size_y = 1.0;        

                let player_instance_data = InstanceData {
                    transform: Renderer::create_transform_matrix(
                        game_state.player_x,
                        game_state.player_y,
                        scale_x,
                        0.3,
                    ),
                    sprite_index: game_state.sprite_index as f32,
                    _padding1: 0.0,
                    sprite_size: [sprite_size_x, sprite_size_y],
                    uv_offset: [0.0, 0.0],
                    uv_scale: [1.0, 1.0],
                };
                player_instances.push(player_instance_data);

                // Calculate instance buffer offsets
                let instance_size = std::mem::size_of::<InstanceData>() as wgpu::BufferAddress;
                let tile_instances_len = tile_instances.len() as wgpu::BufferAddress;
                let player_instances_len = player_instances.len() as wgpu::BufferAddress;
                let tile_instances_size = tile_instances_len * instance_size;
                let player_instances_size = player_instances_len * instance_size;

                // Update instance buffer for tiles
                if tile_instances_size > 0 {
                    renderer.queue.write_buffer(
                        &renderer.instance_buffer,
                        0,
                        bytemuck::cast_slice(&tile_instances),
                    );
                }

                // Update instance buffer for player
                if player_instances_size > 0 {
                    renderer.queue.write_buffer(
                        &renderer.instance_buffer,
                        tile_instances_size,
                        bytemuck::cast_slice(&player_instances),
                    );
                }

                // Get the output frame
                let output = match renderer.surface.get_current_texture() {
                    Ok(output) => output,
                    Err(e) => {
                        eprintln!("Failed to acquire next swap chain texture: {:?}", e);
                        return;
                    }
                };
                let view = output
                    .texture
                    .create_view(&wgpu::TextureViewDescriptor::default());
                let mut encoder = renderer
                    .device
                    .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                        label: Some("Render Encoder"),
                    });

                {
                    let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                        label: Some("Render Pass"),
                        color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                            view: &view,
                            resolve_target: None,
                            ops: wgpu::Operations {
                                load: wgpu::LoadOp::Clear(wgpu::Color {
                                    r: 0.1,
                                    g: 0.2,
                                    b: 0.3,
                                    a: 1.0,
                                }),
                                store: true,
                            },
                        })],
                        depth_stencil_attachment: None,
                    });

                    // Draw tiles
                    if tile_instances_len > 0 {
                        // Set pipeline and bind groups
                        render_pass.set_pipeline(&renderer.pipeline);
                        render_pass.set_bind_group(0, &renderer.tileset_bind_group, &[]);

                        // Set vertex buffers
                        render_pass.set_vertex_buffer(0, renderer.vertex_buffer.slice(..));
                        render_pass.set_vertex_buffer(1, renderer.instance_buffer.slice(0..tile_instances_size));

                        // Set index buffer
                        render_pass.set_index_buffer(
                            renderer.index_buffer.slice(..),
                            wgpu::IndexFormat::Uint16,
                        );

                        // Draw tile instances
                        render_pass.draw_indexed(
                            0..renderer.num_indices,
                            0,
                            0..tile_instances_len as u32,
                        );
                    }

                    // Draw player
                    if player_instances_len > 0 {
                        // Bind the character texture
                        render_pass.set_bind_group(0, &renderer.texture_bind_group, &[]);

                        // Set vertex buffers
                        render_pass.set_vertex_buffer(0, renderer.vertex_buffer.slice(..));
                        render_pass.set_vertex_buffer(1, renderer.instance_buffer.slice(tile_instances_size..(tile_instances_size + player_instances_size)));

                        // Set index buffer
                        render_pass.set_index_buffer(
                            renderer.index_buffer.slice(..),
                            wgpu::IndexFormat::Uint16,
                        );

                        // Draw player instances
                        render_pass.draw_indexed(
                            0..renderer.num_indices,
                            0,
                            0..player_instances_len as u32,
                        );
                    }
                }

                // Submit commands and present frame
                renderer.queue.submit(Some(encoder.finish()));
                output.present();

                // Frame limiting for consistent rendering (60 FPS)
                let frame_duration = std::time::Duration::from_secs_f32(1.0 / 60.0);
                std::thread::sleep(frame_duration.saturating_sub(now.elapsed()));
            }
            _ => {}
        }
    });
}
