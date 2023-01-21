#![no_std]

mod helper;
mod parser;

use aidoku::{
	error::Result,
	prelude::*,
	std::net::{HttpMethod, Request},
	std::{String, Vec},
	Chapter, DeepLink, Filter, Listing, Manga, MangaPageResult, Page,
};

use helper::get_manga_url;
use parser::*;

const URL: &str = "https://mangakatana.com";

#[get_manga_list]
fn get_manga_list(filters: Vec<Filter>, page: i32) -> Result<MangaPageResult> {
	let url = format!("{}/manga/page/{}", URL, page);

	let html = Request::new(url, HttpMethod::Get)
		.html()
		.expect("Failed to get html from mangakatana");

	Ok(parse_manga_list(html, String::from(URL)))
}

#[get_manga_listing]
fn get_manga_listing(listing: Listing, page: i32) -> Result<MangaPageResult> {
	todo!()
}

#[get_manga_details]
fn get_manga_details(manga_id: String) -> Result<Manga> {
	let url = get_manga_url(manga_id, String::from(URL));

	let html = Request::new(url.clone(), HttpMethod::Get)
		.html()
		.expect("Failed to get html from mangakatana");

	Ok(parse_manga_details(html, url, String::from(URL)))
}

#[get_chapter_list]
fn get_chapter_list(manga_id: String) -> Result<Vec<Chapter>> {
	let url = get_manga_url(manga_id, String::from(URL));

	let html = Request::new(url.clone(), HttpMethod::Get)
		.html()
		.expect("Failed to get html from mangakatana");

	Ok(parse_chapter_list(html, String::from(URL)))
}

#[get_page_list]
fn get_page_list(manga_id: String, chapter_id: String) -> Result<Vec<Page>> {
	todo!()
}

#[modify_image_request]
fn modify_image_request(request: Request) {
	request.header("Referer", URL);
}

#[handle_url]
fn handle_url(url: String) -> Result<DeepLink> {
	todo!()
}
