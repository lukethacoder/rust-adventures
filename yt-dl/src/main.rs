use rustube::{Id, VideoFetcher};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let url = "https://youtu.be/4WHBed4d4Uw";
    // println!("downloaded video to {:?}", rustube::download_best_quality(&url).await.unwrap());
    
    let path_to_video = rustube::download_best_quality(url).await?;
    println!("downloaded video to {:?}", path_to_video);
    // let id = Id::from_raw(url).unwrap();
    // let descrambler = VideoFetcher::from_id(id.into_owned())
    //         .unwrap()
    //         .fetch()
    //         .await
    //         .unwrap();
    
    // let view_count = descrambler.video_details().view_count;
    // let title = descrambler.video_title();
    // println!("The video `{}` was viewed {} times.", title, view_count);


    Ok(())
}