use crate::state::State;

pub fn draw(state: &State) {
    let frame = state.surface.get_current_texture().unwrap();

    let view = frame
        .texture
        .create_view(&wgpu::TextureViewDescriptor::default());

    let mut encoder = state
        .device
        .create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("Command Encoder"),
        });

    fill_vertex_buffer_data(state);

    {
        let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("Vertex Render Pass"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: &view,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(wgpu::Color::WHITE),
                    store: wgpu::StoreOp::Store,
                },
            })],
            depth_stencil_attachment: None,
            timestamp_writes: None,
            occlusion_query_set: None,
        });

        render_pass.set_pipeline(&state.render_pipeline);
        render_pass.draw(0..3, 0..1);
    }
    state.queue.submit(Some(encoder.finish()));
    frame.present();
}

fn fill_vertex_buffer_data(state: &crate::state::State) {
    let mut vertex_buffer_data = state.vertex_buffer.slice(..).get_mapped_range_mut();
    vertex_buffer_data.fill(0);

    let mut offset = 0;

    for instance in &state.instances {
        if let Some(mesh) = &instance.mesh {
            let vertices = crate::models::into_vertex_vec(&mesh.vertices);
            let slice = bytemuck::cast_slice(&vertices);

            state
                .queue
                .write_buffer(&state.vertex_buffer, offset, slice);
            offset += slice.len() as u64;
        }
    }
}
