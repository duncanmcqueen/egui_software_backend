use egui::{Color32, ColorImage, TextureId, TextureOptions, epaint::ImageDelta};
use egui_software_backend::{BufferMutRef, ColorFieldOrder, EguiSoftwareRender};

#[test]
fn cached_mesh_reflects_full_and_partial_texture_updates() {
    let id = TextureId::Managed(42);
    let mut mesh = egui::Mesh::with_texture(id);
    let rect = egui::Rect::from_min_max(egui::pos2(0.0, 0.0), egui::pos2(8.0, 8.0));
    mesh.add_rect_with_uv(
        rect,
        egui::Rect::from_min_max(egui::pos2(0.0, 0.0), egui::pos2(1.0, 1.0)),
        Color32::WHITE,
    );
    let primitives = [egui::ClippedPrimitive {
        clip_rect: rect,
        primitive: egui::epaint::Primitive::Mesh(mesh),
    }];
    let mut renderer = EguiSoftwareRender::new(ColorFieldOrder::Rgba).with_caching(true);
    let mut pixels = vec![[0; 4]; 64];
    let mut draw = |delta: Option<ImageDelta>| {
        let mut textures = egui::TexturesDelta::default();
        if let Some(delta) = delta {
            textures.set.push((id, delta));
        }
        renderer.render(
            &mut BufferMutRef::new(&mut pixels, 8, 8),
            &primitives,
            &textures,
            1.0,
        );
        pixels.clone()
    };
    let red = Color32::from_rgb(255, 0, 0);
    let blue = Color32::from_rgb(0, 0, 255);
    let green = Color32::from_rgb(0, 255, 0);
    let full = |color| ImageDelta::full(ColorImage::filled([2, 2], color), TextureOptions::NEAREST);
    assert_eq!(draw(Some(full(red)))[6 + 6 * 8], red.to_array());
    assert_eq!(draw(Some(full(blue)))[6 + 6 * 8], blue.to_array());
    let partial = ImageDelta::partial(
        [1, 1],
        ColorImage::filled([1, 1], green),
        TextureOptions::NEAREST,
    );
    let pixels = draw(Some(partial));
    assert_eq!(pixels[6 + 6 * 8], green.to_array());
    assert_eq!(pixels[1 + 8], blue.to_array());
    assert_eq!(draw(None), pixels);
}
