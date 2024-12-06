use crate::engine::{GameState, InputHandler, Renderer};
use crate::engine::renderer::tile::TileMap;
use crate::engine::renderer::instance::InstanceData;
use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};
use pollster::block_on;

/// Runs the main game loop, initializing the window, handling events, and rendering frames.
/// Runs the main game loop, initializing the window, handling events, and rendering frames.
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

    // Calculate scaling factors for each background layer based on their image sizes
    let window_width = window.inner_size().width as f32;
    let window_height = window.inner_size().height as f32;

    let mut background_instances = Vec::new();

    for (i, bg_texture) in renderer.background_textures.iter().enumerate() {
        let background_scale_x = window_width / bg_texture.width as f32;
        let background_scale_y = window_height / bg_texture.height as f32;

        let z = 1.0 - (i as f32 * 0.2); // Example: Furthest layer at z=1.0, closer layers decreasing z

        background_instances.push(InstanceData {
            transform: Renderer::create_transform_matrix(
                0.0,                  // x position
                0.0,                  // y position
                z,                    // z depth
                background_scale_x,   // scale_x to fill the window
                background_scale_y,   // scale_y to fill the window
            ),
            sprite_index: 0.0,
            _padding1: 0.0,
            sprite_size: [1.0, 1.0],
            uv_offset: [0.0, 0.0],
            uv_scale: [1.0, 1.0],
        });
    }

    // Timing variables for frame timing
    let mut last_frame_time = std::time::Instant::now();

    // Run the event loop
    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Poll; // Keep the event loop running

        match event {
            Event::WindowEvent { event, .. } => handle_window_event(event, control_flow, &mut input_handler),
            Event::MainEventsCleared => {
                let delta_time = update_game_state(&mut game_state, &input_handler, &mut last_frame_time);

                let (tile_instances, player_instances) = prepare_instances(&tile_map, &game_state, &renderer);

                update_instance_buffers(
                    &renderer,
                    &background_instances,
                    &tile_instances,
                    &player_instances,
                );

                render_frame(&renderer, &tile_instances, &player_instances);

                // Frame limiting for consistent rendering (60 FPS)
                let frame_duration = std::time::Duration::from_secs_f32(1.0 / 60.0);
                std::thread::sleep(frame_duration.saturating_sub(last_frame_time.elapsed()));
            }
            _ => {}
        }
    });
}


/// Handles window-related events such as closing the application and keyboard input.
///
/// # Arguments
///
/// * event - The event triggered by the window.
/// * control_flow - Used to control the flow of the event loop.
/// * input_handler - The input handler to update with keyboard inputs.
fn handle_window_event(
    event: WindowEvent,
    control_flow: &mut ControlFlow,
    input_handler: &mut InputHandler,
) {
    match event {
        WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
        WindowEvent::KeyboardInput { input, .. } => input_handler.handle_keyboard_input(input),
        _ => {}
    }
}

/// Updates the game state, including handling input, physics, and animation.
///
/// # Arguments
///
/// * game_state - The current state of the game.
/// * input_handler - Provides the current input state.
/// * last_frame_time - Tracks the time of the last frame for calculating delta time.
///
/// # Returns
///
/// The time delta between the current and the last frame.
fn update_game_state(
    game_state: &mut GameState,
    input_handler: &InputHandler,
    last_frame_time: &mut std::time::Instant,
) -> f32 {
    let now = std::time::Instant::now();
    let delta_time = now.duration_since(*last_frame_time).as_secs_f32();
    *last_frame_time = now;

    game_state.update(input_handler, delta_time);

    delta_time
}

/// Prepares the instance data for tiles and the player for rendering.
///
/// # Arguments
///
/// * tile_map - The tile map containing all tiles.
/// * game_state - The current state of the game.
/// * renderer - The renderer for accessing tile and texture details.
///
/// # Returns
///
/// A tuple containing vectors of instance data for tiles and the player.
fn prepare_instances(
    tile_map: &TileMap,
    game_state: &GameState,
    renderer: &Renderer,
) -> (Vec<InstanceData>, Vec<InstanceData>) {
    let mut tile_instances = Vec::new();
    let mut player_instances = Vec::new();

    // Prepare tile instances
    for tile in &tile_map.tiles {
        let tile_size_u = 1.0 / renderer.tileset_columns as f32;
        let tile_size_v = 1.0 / renderer.tileset_rows as f32;
        let u = (tile.tile_index % renderer.tileset_columns) as f32 * tile_size_u;
        let v = (tile.tile_index / renderer.tileset_columns) as f32 * tile_size_v;
        let uv_offset = [u, v];
        let uv_scale = [tile_size_u, tile_size_v];

        let tile_z = 0.0; // Ground level
        let tile_scale_x = tile_map.tile_width; // e.g., 1.0
        let tile_scale_y = tile_map.tile_height; // e.g., 1.0

        tile_instances.push(InstanceData {
            transform: Renderer::create_transform_matrix(
                tile.position.0,
                tile.position.1,
                tile_z,
                tile_scale_x,
                tile_scale_y,
            ),
            sprite_index: 0.0,
            _padding1: 0.0,
            sprite_size: [0.0, 0.0],
            uv_offset,
            uv_scale,
        });
    }

    // Prepare player instance
    let player_z = -0.5; // In front of tiles
    let scale_x = if game_state.facing_right { 0.3 } else { -0.3 };
    let scale_y = 0.3; // Non-zero scaling

    // Calculate UV offset and scale for player
    let sprite_width = 1.0 / 24.0; // Fixed sprite width (24 columns in the tileset)
    let sprite_height = 1.0;      // Full height for a single sprite
    let uv_offset = [0.0, 0.0];   // Hardcoded to match the working code
    let uv_scale = [1.0, 1.0];    // Matches the entire texture dimensions

    player_instances.push(InstanceData {
        transform: Renderer::create_transform_matrix(
            game_state.player_x,
            game_state.player_y,
            player_z,
            scale_x,
            scale_y,
        ),
        sprite_index: game_state.sprite_index as f32,
        _padding1: 0.0,
        sprite_size: [sprite_width, sprite_height],
        uv_offset,
        uv_scale,
    });

    (tile_instances, player_instances)
}




/// Updates the instance buffer data for the renderer.
///
/// # Arguments
///
/// * renderer - The renderer to update the buffers for.
/// * background_instances - Instance data for the background layers.
/// * tile_instances - Instance data for tiles.
/// * player_instances - Instance data for the player.
fn update_instance_buffers(
    renderer: &Renderer,
    background_instances: &[InstanceData],
    tile_instances: &[InstanceData],
    player_instances: &[InstanceData],
) {
    let instance_size = std::mem::size_of::<InstanceData>() as wgpu::BufferAddress;

    // Calculate buffer offsets
    let background_instances_size = background_instances.len() as wgpu::BufferAddress * instance_size;
    let tile_instances_size = tile_instances.len() as wgpu::BufferAddress * instance_size;
    let player_instances_size = player_instances.len() as wgpu::BufferAddress * instance_size;

    // Write background instances
    if !background_instances.is_empty() {
        renderer.queue.write_buffer(
            &renderer.instance_buffer,
            0,
            bytemuck::cast_slice(background_instances),
        );
    }

    // Write tile instances
    if !tile_instances.is_empty() {
        renderer.queue.write_buffer(
            &renderer.instance_buffer,
            background_instances_size,
            bytemuck::cast_slice(tile_instances),
        );
    }

    // Write player instances
    if !player_instances.is_empty() {
        renderer.queue.write_buffer(
            &renderer.instance_buffer,
            background_instances_size + tile_instances_size,
            bytemuck::cast_slice(player_instances),
        );
    }
}



/// Renders a frame by issuing draw calls to the GPU.
///
/// # Arguments
///
/// * renderer - The renderer to use for drawing.
/// * tile_instances - Instance data for tiles.
/// * player_instances - Instance data for the player.
fn render_frame(
    renderer: &Renderer,
    tile_instances: &[InstanceData],
    player_instances: &[InstanceData],
) {
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
    let depth_view = renderer
        .depth_texture
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
            depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                view: &depth_view,
                depth_ops: Some(wgpu::Operations {
                    load: wgpu::LoadOp::Clear(1.0),
                    store: true,
                }),
                stencil_ops: None,
            }),
        });

        // Ensure index buffer is bound
        render_pass.set_index_buffer(
            renderer.index_buffer.slice(..),
            wgpu::IndexFormat::Uint16,
        );

        // Render background layers
        for (i, bind_group) in renderer.background_bind_groups.iter().enumerate() {
            let offset = i as wgpu::BufferAddress * std::mem::size_of::<InstanceData>() as wgpu::BufferAddress;
            render_pass.set_vertex_buffer(
                1,
                renderer.instance_buffer.slice(
                    offset..offset + std::mem::size_of::<InstanceData>() as wgpu::BufferAddress,
                ),
            );

            render_pass.set_pipeline(&renderer.pipeline);
            render_pass.set_bind_group(0, bind_group, &[]);
            render_pass.set_vertex_buffer(0, renderer.vertex_buffer.slice(..));
            render_pass.set_vertex_buffer(1, renderer.instance_buffer.slice(offset..offset + std::mem::size_of::<InstanceData>() as wgpu::BufferAddress));
            render_pass.draw_indexed(0..renderer.num_indices, 0, 0..1); // Ensure `num_indices` matches `INDICES`
        }

        // Render tiles
        if !tile_instances.is_empty() {
            let background_instances_size = renderer.background_bind_groups.len() as wgpu::BufferAddress
                * std::mem::size_of::<InstanceData>() as wgpu::BufferAddress;

            render_pass.set_pipeline(&renderer.pipeline);
            render_pass.set_bind_group(0, &renderer.tileset_bind_group, &[]);
            render_pass.set_vertex_buffer(0, renderer.vertex_buffer.slice(..));
            render_pass.set_vertex_buffer(
                1,
                renderer
                    .instance_buffer
                    .slice(background_instances_size..background_instances_size + tile_instances.len() as wgpu::BufferAddress * std::mem::size_of::<InstanceData>() as wgpu::BufferAddress),
            );
            render_pass.draw_indexed(
                0..renderer.num_indices,
                0,
                0..tile_instances.len() as u32,
            );
        }

        // Render player
        if !player_instances.is_empty() {
            let background_instances_size = renderer.background_bind_groups.len() as wgpu::BufferAddress
                * std::mem::size_of::<InstanceData>() as wgpu::BufferAddress;
            let tile_instances_size = tile_instances.len() as wgpu::BufferAddress
                * std::mem::size_of::<InstanceData>() as wgpu::BufferAddress;

            render_pass.set_pipeline(&renderer.pipeline);
            render_pass.set_bind_group(0, &renderer.texture_bind_group, &[]);
            render_pass.set_vertex_buffer(0, renderer.vertex_buffer.slice(..));
            render_pass.set_vertex_buffer(
                1,
                renderer
                    .instance_buffer
                    .slice(background_instances_size + tile_instances_size..background_instances_size + tile_instances_size + player_instances.len() as wgpu::BufferAddress * std::mem::size_of::<InstanceData>() as wgpu::BufferAddress),
            );
            render_pass.draw_indexed(
                0..renderer.num_indices,
                0,
                0..player_instances.len() as u32,
            );
        }
    }

    renderer.queue.submit(Some(encoder.finish()));
    output.present();
}

