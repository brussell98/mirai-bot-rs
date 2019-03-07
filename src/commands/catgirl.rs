use serenity::model::channel::Message;
use serenity::prelude::Context;
use serenity::framework::standard::Args;

#[derive(Deserialize)]
struct NekoResponse {
	images: Vec<Catgirl>,
}

#[derive(Deserialize)]
struct Catgirl {
	id: String,
	#[serde(rename = "createdAt")]
	created_at: String,
	likes: i32,
	favorites: i32,
}

command!(catgirl(_ctx, msg, _args) {
	let can_nsfw = msg.channel().unwrap().is_nsfw();

	let url = if can_nsfw {
		"https://nekos.moe/api/v1/random/image"
	} else {
		"https://nekos.moe/api/v1/random/image?nsfw=false"
	};

	let client = reqwest::Client::new();
	let mut res = client.get(url)
		.header(reqwest::header::USER_AGENT, "mirai-bot-rs v0.1.0")
		.send()?;

	if res.status().is_success() {
		let neko_resp: NekoResponse = res.json()?;

		if let Err(why) = msg.channel_id.send_message(|m| m
			.embed(|e| e
				.title("Random Image From Nekos.moe")
				.url(format!("https://nekos.moe/post/{}", neko_resp.images[0].id))
				.color(9874412)
				.timestamp(neko_resp.images[0].created_at.to_string())
				.footer(|f| f
					.icon_url("https://nekos.moe/static/favicon/favicon-32x32.png")
					.text("Image from Nekos.moe"))
				.image(format!("https://nekos.moe/image/{}", neko_resp.images[0].id))
				.field("Likes", neko_resp.images[0].likes, true)
				.field("Favorites", neko_resp.images[0].favorites, true))) {
			println!("Error sending message: {:?}", why);
		}
	} else {
		println!("Something went wrong. status: {:?}", res.status());

		if let Err(why) = msg.channel_id.say("Something went wrong!") {
			println!("Error sending message: {:?}", why);
		}
	}
});
