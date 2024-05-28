use crate::state::State;

pub fn draw(state: &mut State) {
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
    let index_count = fill_index_buffer_data(state);

    state.camera_uniform.update_view_proj(&state.camera);
    state.queue.write_buffer(
        &state.camera_buffer,
        0,
        bytemuck::cast_slice(&[state.camera_uniform]),
    );

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

        render_pass.set_bind_group(0, &state.camera_bind_group, &[]);
        render_pass.set_pipeline(&state.render_pipeline);
        render_pass.set_vertex_buffer(0, state.vertex_buffer.slice(..));
        render_pass.set_index_buffer(state.index_buffer.slice(..), wgpu::IndexFormat::Uint16);
        render_pass.draw_indexed(0..index_count as u32, 0, 0..1);
    }

    state.queue.submit(Some(encoder.finish()));
    frame.present();
}

fn fill_vertex_buffer_data(state: &crate::state::State) {
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

fn fill_index_buffer_data(state: &crate::state::State) -> u64 {
    let mut offset = 0;

    for instance in &state.instances {
        if let Some(mesh) = &instance.mesh {
            let slice = bytemuck::cast_slice(&mesh.indices);

            state.queue.write_buffer(&state.index_buffer, offset, slice);
            offset += slice.len() as u64;
        }
    }

    offset
}
