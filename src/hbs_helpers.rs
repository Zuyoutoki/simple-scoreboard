use rocket_dyn_templates::handlebars::handlebars_helper;

handlebars_helper!(contains: |needle: i64, haystack: Vec<i64>| haystack.contains(&needle));
handlebars_helper!(plus_one: |number: i64| number+1);
