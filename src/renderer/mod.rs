use crate::state::State;
use std::ops::Range;

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
        render_pass.draw_mesh(state.instances.first().unwrap().model.mesh.first().unwrap())
    }

    state.queue.submit(Some(encoder.finish()));
    frame.present();
}

pub trait DrawModel<'a> {
    fn draw_mesh(&mut self, mesh: &'a crate::models::Mesh);
    fn draw_mesh_instanced(&mut self, mesh: &'a crate::models::Mesh, instances: Range<u32>);
}

impl<'a, 'b> DrawModel<'b> for wgpu::RenderPass<'a>
where
    'b: 'a,
{
    fn draw_mesh(&mut self, mesh: &'b crate::models::Mesh) {
        self.draw_mesh_instanced(mesh, 0..1)
    }

    fn draw_mesh_instanced(&mut self, mesh: &'b crate::models::Mesh, instances: Range<u32>) {
        self.set_vertex_buffer(0, mesh.vertex_buffer.slice(..));
        self.set_index_buffer(mesh.index_buffer.slice(..), wgpu::IndexFormat::Uint32);
        self.draw_indexed(0..mesh.num_elements, 0, instances);
    }
}
