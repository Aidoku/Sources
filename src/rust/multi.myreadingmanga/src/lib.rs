#![no_std]
use aidoku::{
	error::Result,
	prelude::*,
	std::{net::Request, String, Vec},
	Chapter, DeepLink, Filter, Manga, MangaPageResult, Page,
};

mod parser;
use parser::{get_filtered_url, request_get, BASE_URL, USER_AGENT};

#[get_manga_list]
fn get_manga_list(filters: Vec<Filter>, page: i32) -> Result<MangaPageResult> {
	let url = get_filtered_url(filters, page);
	let html = request_get(url).html()?;

	parser::get_manga_list(html, page)
}

#[get_manga_details]
fn get_manga_details(id: String) -> Result<Manga> {
	let url = format!("{}{}/", BASE_URL, id);
	let html = request_get(url).html()?;

	parser::get_manga_details(html, id)
}

#[get_chapter_list]
fn get_chapter_list(id: String) -> Result<Vec<Chapter>> {
	let url = format!("{}{}/", BASE_URL, id);
	let html = request_get(url).html()?;

	parser::get_chapter_list(html, id)
}

#[get_page_list]
fn get_page_list(_manga_id: String, _chapter_id: String) -> Result<Vec<Page>> {
	todo!()
}

#[modify_image_request]
fn modify_image_request(request: Request) {
	request
		.header("Referer", BASE_URL)
		.header("User-Agent", USER_AGENT);
}

#[handle_url]
fn handle_url(_url: String) -> Result<DeepLink> {
	todo!()
}
