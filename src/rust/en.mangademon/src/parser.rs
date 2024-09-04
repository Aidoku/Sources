use aidoku::{
	std::{html::Node, String, Vec},
	Chapter, Manga, MangaContentRating, MangaPageResult, MangaStatus, MangaViewer, Page,
};

use crate::helper::*;

pub fn parse_latest_manga_list(html: Node) -> MangaPageResult {
	let mut manga: Vec<Manga> = Vec::new();

	for node in html.select(".updates-element").array() {
		let node = node.as_node().expect("Failed to get node");

		let cover_node = node.select("img").first();
		let cover = cover_node.attr("abs:src").read();
		let title = cover_node.attr("title").read();

		let link_node = node.select("a").first();
		let raw_url = link_node.attr("href").read();
		let id = get_manga_id(&raw_url);
		let url = get_manga_url(&id);

		manga.push(Manga {
			id,
			cover,
			title,
			url,
			..Default::default()
		});
	}

	let has_more = !manga.is_empty();

	MangaPageResult { manga, has_more }
}

pub fn parse_manga_list(html: Node, searching: bool) -> MangaPageResult {
	let mut manga: Vec<Manga> = Vec::new();

	if searching {
		for node in html.select("a").array() {
			let node = node.as_node().expect("Failed to get node");

			let raw_url = node.attr("href").read();
			let id = get_manga_id(&raw_url);
			let url = get_manga_url(&id);

			let cover_node = node.select("img").first();
			let cover = cover_node.attr("abs:src").read();

			let title_node = node.select("div:first-child").first();
			let title = String::from(title_node.text().read().trim());

			manga.push(Manga {
				id,
				cover,
				title,
				url,
				..Default::default()
			})
		}

		return MangaPageResult {
			manga,
			has_more: false,
		};
	}

	for node in html.select(".advanced-element").array() {
		let node = node
			.as_node()
			.expect("Failed to get node")
			.select("a")
			.first();

		let raw_url = node.attr("href").read();

		let title = node.attr("title").read();
		let id = get_manga_id(&raw_url);
		let url = get_manga_url(&id);
		let cover = node.select("img").attr("abs:src").read();

		manga.push(Manga {
			id,
			cover,
			title,
			url,
			..Default::default()
		});
	}

	let has_more = !manga.is_empty();

	MangaPageResult { manga, has_more }
}

pub fn parse_manga_details(html: Node, manga_url: String) -> Manga {
	let id = get_manga_id(&manga_url);

	let title = html.select(".big-fat-titles").first().text().read();

	let mut categories = Vec::new();
	let genre_list_node = html.select(".genres-list").first();
	for node in genre_list_node.select("li").array() {
		let node = node.as_node().expect("Failed to get genre node");
		let genre = node.text().read();
		categories.push(genre);
	}

	let description = html.select(".white-font").first().text().read();

	let mut author = String::from("");
	let mut artist = String::from("");
	let mut status = String::from("");

	let manga_info_node = html.select("#manga-info-stats").first();
	for node in manga_info_node.select("#manga-info-stats > div").array() {
		let node = node.as_node().expect("Failed to get manga info node");

		let label = node.select("li").first().text().read();
		let value = node.select("li").last().text().read();

		match label.as_str() {
			"Author" => author = value,
			"Status" => status = value,
			// Artist doesn't exist at the time of writing this but it's here for future proofing
			"Artist" => artist = value,
			_ => {}
		}
	}

	let status = match status.to_lowercase().trim() {
		"ongoing" => MangaStatus::Ongoing,
		"completed" => MangaStatus::Completed,
		"cancelled" => MangaStatus::Cancelled,
		"hiatus" => MangaStatus::Hiatus,
		_ => MangaStatus::Unknown,
	};

	let nsfw = {
		let mut rating = MangaContentRating::Safe;
		for genre in categories.iter() {
			match genre.to_lowercase().trim() {
				"Ecchi" | "Harem" | "Mature" => rating = MangaContentRating::Suggestive,
				"Smut" => rating = MangaContentRating::Nsfw,
				_ => {}
			}
		}
		rating
	};

	let cover_node = html.select("img.border-box").first();
	let cover = cover_node.attr("abs:src").read();

	let viewer = MangaViewer::Scroll;

	Manga {
		id,
		title,
		categories,
		description,
		author,
		artist,
		status,
		nsfw,
		cover,
		viewer,
		..Default::default()
	}
}

pub fn parse_chapter_list(html: Node) -> Vec<Chapter> {
	let mut chapters: Vec<Chapter> = Vec::new();

	for node in html
		.select("#novel .novel-header #mangainfo #chapters .chapter-list li")
		.array()
	{
		let node = node.as_node().expect("Failed to get chapter node");

		let raw_url = node.select("a").attr("href").read();
		let id = node.attr("data-chapterno").read();

		let manga_id = get_manga_id(&raw_url);
		let url = get_chapter_url(&id, &manga_id);

		let chapter = node
			.attr("data-chapterno")
			.read()
			.parse::<f32>()
			.unwrap_or(-1.0);

		let volume = node
			.attr("data-volumeno")
			.read()
			.parse::<f32>()
			.unwrap_or(-1.0);

		let date_updated =
			node.select(".chapter-update")
				.attr("date")
				.as_date("yyyy-MM-dd", Some("en-US"), None);

		chapters.push(Chapter {
			id,
			chapter,
			volume: if volume == 0.0 { -1.0 } else { volume },
			date_updated,
			url,
			..Default::default()
		});
	}

	chapters
}

pub fn parse_page_list(html: Node) -> Vec<Page> {
	let mut pages: Vec<Page> = Vec::new();

	for node in html.select("main .wrapper center img.imgholder").array() {
		let node = node.as_node().expect("Failed to get image node");

		let url = node.attr("src").read();

		let index = url
			.split('/')
			.last()
			.and_then(|part| part.split('.').next())
			.and_then(|part| part.parse::<i32>().ok())
			.unwrap_or(-1);

		pages.push(Page {
			index,
			url,
			..Default::default()
		});
	}

	pages
}
