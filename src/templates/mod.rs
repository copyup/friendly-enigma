use askama::Template;
use crate::models::{CategoryWithChildren, Page, Post};

#[derive(Template)]
#[template(path = "index.html")]
pub struct IndexTemplate {
    pub posts: Vec<Post>,
}

#[derive(Template)]
#[template(path = "posts.html")]
pub struct PostsTemplate {
    pub posts: Vec<Post>,
}

#[derive(Template)]
#[template(path = "post.html")]
pub struct PostTemplate {
    pub post: Post,
    pub content_html: String,
    pub published_at: String,
    pub updated_at: String,
}

#[derive(Template)]
#[template(path = "categories.html")]
pub struct CategoriesTemplate {
    pub categories: Vec<CategoryWithChildren>,
}

#[derive(Template)]
#[template(path = "page.html")]
pub struct PageTemplate {
    pub page: Page,
    pub content_html: String,
}
