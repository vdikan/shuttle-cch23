use image::GenericImageView;
use axum::extract::Multipart;
use axum::response::Result as ShuttleResult;

pub async fn day11_reds(mut multipart: Multipart) -> ShuttleResult<String> {
    let mut reds = 0;

    let Some(field) = multipart.next_field().await.unwrap() else {
        return Ok(String::from("No data"));
    };
    let data = field.bytes().await?;
    let Ok(img) = image::load_from_memory(&data) else {
        return Ok(String::from("No valid image"));
    };

    let count = img
        .pixels()
        .filter(|(_i, _j, v)| {
            let r = u16::from(v.0[0]);
            let g = u16::from(v.0[1]);
            let b = u16::from(v.0[2]);

            r > g + b
        })
        .count();

    reds += count;

    Ok(reds.to_string())
}
