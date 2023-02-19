use aidoku::
{
	std::String,
	MangaContentRating
};

pub fn IsNSFW(genre: String) -> MangaContentRating{

	let NSFWCategories: String = String::from("Еччі Юрі Яой"); // maybe new
	if NSFWCategories.contains(&genre)
	{
		return MangaContentRating::Nsfw;
	}
	return MangaContentRating::Safe;
}

pub fn IsNSFWBool(genre: String) -> bool {
	let NSFWCategories: String = String::from("Еччі Юрі Яой"); // maybe new
	if NSFWCategories.contains(&genre)
	{
		return true;
	}
	return false;
}

pub fn GetStatusString(status: String) -> String{
	if status == "Триває"{
		return String::from("Ongoing");
	}
	if status == "Закінчений"{
		return String::from("Completed");
	}
	if status == "Невідомо"{
		return String::from("Unknown");
	}
	if status == "Покинуто"{
		return String::from("Cancelled");
	}
	return String::from("Unknown"); // find others
}

pub fn genres_list() -> [&'static str; 50] {
	[
		"",
		"dementia",
		"boyovik",
		"boyov-mistectva",
		"budenst",
		"vampri",
		"garem",
		"kodomo",
		"detektiv",
		"demons",
		"josei",
		"doujinshi",
		"drama",
		"ecchi",
		"zhahi",
		"gender-bender",
		"games",
		"storia",
		"yonkoma",
		"space",
		"komedia",
		"maho-shoujou",
		"cars",
		"meha",
		"mstika",
		"music",
		"nadprirodne",
		"naukova-fantastika",
		"parody",
		"prigodi",
		"psihologia",
		"police",
		"postapokalptika",
		"romantika",
		"samurai",
		"sentai",
		"seinen",
		"sport",
		"superpower",
		"tragedia",
		"triler",
		"fantastika",
		"fentez",
		"shoujo",
		"shoujo-ai",
		"shounen",
		"shounen-ai",
		"shkola",
		"iur",
		"shonen-ay",
	]
}
