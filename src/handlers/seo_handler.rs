use actix_web::{web, HttpResponse};
use chrono::Utc;

use crate::db::{PageRepository, PostRepository};

const BASE_URL: &str = "https://tinlike.com";
const SITEMAP_MAX_URLS: usize = 50000;

pub async fn robots_txt() -> HttpResponse {
    let content = format!(r#"User-agent: *
Allow: /
Disallow: /api/

Sitemap: {}/sitemap-index.xml
"#, BASE_URL);
    HttpResponse::Ok()
        .content_type("text/plain")
        .body(content)
}

/// Sitemap 索引文件
pub async fn sitemap_index(
    post_repo: web::Data<PostRepository>,
    page_repo: web::Data<PageRepository>,
) -> Result<HttpResponse, actix_web::Error> {
    // 获取数量，不需要完整数据
    let post_count = match post_repo.get_published_count().await {
        Ok(c) => c,
        Err(e) => return Ok(HttpResponse::InternalServerError().body(format!("Error: {}", e))),
    };

    let page_count = match page_repo.get_published_count().await {
        Ok(c) => c,
        Err(e) => return Ok(HttpResponse::InternalServerError().body(format!("Error: {}", e))),
    };

    let now = Utc::now().format("%Y-%m-%d").to_string();

    // 预分配容量
    let mut sitemap = String::with_capacity(1024);
    sitemap.push_str(r#"<?xml version="1.0" encoding="UTF-8"?>
<sitemapindex xmlns="http://www.sitemaps.org/schemas/sitemap/0.9">
"#);

    // 静态页面 sitemap
    sitemap.push_str(&format!(r#"  <sitemap>
    <loc>{}/sitemap-static.xml</loc>
    <lastmod>{}</lastmod>
  </sitemap>
"#, BASE_URL, now));

    // 文章 sitemap（分页）
    let post_sitemaps = (post_count + SITEMAP_MAX_URLS as i64 - 1) / SITEMAP_MAX_URLS as i64;
    for i in 0..post_sitemaps {
        sitemap.push_str(&format!(r#"  <sitemap>
    <loc>{}/sitemap-posts-{}.xml</loc>
    <lastmod>{}</lastmod>
  </sitemap>
"#, BASE_URL, i, now));
    }

    // 页面 sitemap
    if page_count > 0 {
        sitemap.push_str(&format!(r#"  <sitemap>
    <loc>{}/sitemap-pages.xml</loc>
    <lastmod>{}</lastmod>
  </sitemap>
"#, BASE_URL, now));
    }

    sitemap.push_str("</sitemapindex>");

    Ok(HttpResponse::Ok()
        .content_type("application/xml")
        .body(sitemap))
}

/// 静态页面 Sitemap
pub async fn sitemap_static() -> HttpResponse {
    let now = Utc::now().format("%Y-%m-%d").to_string();

    let mut sitemap = String::with_capacity(2048);
    sitemap.push_str(r#"<?xml version="1.0" encoding="UTF-8"?>
<urlset xmlns="http://www.sitemaps.org/schemas/sitemap/0.9">
"#);

    // 首页
    sitemap.push_str(&format!(r#"  <url>
    <loc>{}/</loc>
    <lastmod>{}</lastmod>
    <changefreq>daily</changefreq>
    <priority>1.0</priority>
  </url>
"#, BASE_URL, now));

    // 文章列表页
    sitemap.push_str(&format!(r#"  <url>
    <loc>{}/posts</loc>
    <lastmod>{}</lastmod>
    <changefreq>daily</changefreq>
    <priority>0.8</priority>
  </url>
"#, BASE_URL, now));

    // 分类页
    sitemap.push_str(&format!(r#"  <url>
    <loc>{}/categories</loc>
    <lastmod>{}</lastmod>
    <changefreq>weekly</changefreq>
    <priority>0.6</priority>
  </url>
"#, BASE_URL, now));

    sitemap.push_str("</urlset>");

    HttpResponse::Ok()
        .content_type("application/xml")
        .body(sitemap)
}

/// 文章 Sitemap（分页）
pub async fn sitemap_posts(
    path: web::Path<i64>,
    post_repo: web::Data<PostRepository>,
) -> Result<HttpResponse, actix_web::Error> {
    let page = path.into_inner();
    let offset = page * SITEMAP_MAX_URLS as i64;

    let posts = match post_repo.get_published_paginated(SITEMAP_MAX_URLS as i64, offset).await {
        Ok(p) => p,
        Err(e) => return Ok(HttpResponse::InternalServerError().body(format!("Error: {}", e))),
    };

    // 预分配容量，每个URL大约200字节
    let mut sitemap = String::with_capacity(posts.len() * 200);
    sitemap.push_str(r#"<?xml version="1.0" encoding="UTF-8"?>
<urlset xmlns="http://www.sitemaps.org/schemas/sitemap/0.9">
"#);

    for post in posts {
        let lastmod = post.updated_at.format("%Y-%m-%d").to_string();
        sitemap.push_str(&format!(r#"  <url>
    <loc>{}/posts/{}</loc>
    <lastmod>{}</lastmod>
    <changefreq>weekly</changefreq>
    <priority>0.7</priority>
  </url>
"#, BASE_URL, post.slug, lastmod));
    }

    sitemap.push_str("</urlset>");

    Ok(HttpResponse::Ok()
        .content_type("application/xml")
        .body(sitemap))
}

/// 页面 Sitemap
pub async fn sitemap_pages(
    page_repo: web::Data<PageRepository>,
) -> Result<HttpResponse, actix_web::Error> {
    let pages = match page_repo.get_published().await {
        Ok(p) => p,
        Err(e) => return Ok(HttpResponse::InternalServerError().body(format!("Error: {}", e))),
    };

    // 预分配容量
    let mut sitemap = String::with_capacity(pages.len() * 200);
    sitemap.push_str(r#"<?xml version="1.0" encoding="UTF-8"?>
<urlset xmlns="http://www.sitemaps.org/schemas/sitemap/0.9">
"#);

    for page in pages {
        let lastmod = page.updated_at.format("%Y-%m-%d").to_string();
        sitemap.push_str(&format!(r#"  <url>
    <loc>{}/pages/{}</loc>
    <lastmod>{}</lastmod>
    <changefreq>monthly</changefreq>
    <priority>0.5</priority>
  </url>
"#, BASE_URL, page.slug, lastmod));
    }

    sitemap.push_str("</urlset>");

    Ok(HttpResponse::Ok()
        .content_type("application/xml")
        .body(sitemap))
}

/// RSS 2.0 Feed
pub async fn rss_xml(
    post_repo: web::Data<PostRepository>,
) -> Result<HttpResponse, actix_web::Error> {
    let posts = match post_repo.get_published_recent(20).await {
        Ok(p) => p,
        Err(e) => return Ok(HttpResponse::InternalServerError().body(format!("Error: {}", e))),
    };

    let build_date = Utc::now().format("%a, %d %b %Y %H:%M:%S GMT").to_string();

    // 预分配容量
    let mut rss = String::with_capacity(4096);
    rss.push_str(r#"<?xml version="1.0" encoding="UTF-8"?>
<rss version="2.0" xmlns:atom="http://www.w3.org/2005/Atom">
  <channel>
    <title>我的博客</title>
    <link>"#);
    rss.push_str(BASE_URL);
    rss.push_str(r#"</link>
    <description>分享技术、生活和思考的博客</description>
    <language>zh-CN</language>
    <lastBuildDate>"#);
    rss.push_str(&build_date);
    rss.push_str(r#"</lastBuildDate>
    <atom:link href=""#);
    rss.push_str(BASE_URL);
    rss.push_str(r#"/rss.xml" rel="self" type="application/rss+xml" />
"#);

    for post in posts {
        let pub_date = post.published_at
            .map(|d| d.format("%a, %d %b %Y %H:%M:%S GMT").to_string())
            .unwrap_or_else(|| build_date.clone());

        let description = post.excerpt.clone()
            .unwrap_or_else(|| post.content.chars().take(200).collect::<String>() + "...");

        rss.push_str(&format!(r#"    <item>
      <title>{}</title>
      <link>{}/posts/{}</link>
      <guid>{}/posts/{}</guid>
      <pubDate>{}</pubDate>
      <description><![CDATA[{}]]></description>
    </item>
"#,
            escape_xml(&post.title),
            BASE_URL, post.slug,
            BASE_URL, post.slug,
            pub_date,
            description
        ));
    }

    rss.push_str(r#"  </channel>
</rss>"#);

    Ok(HttpResponse::Ok()
        .content_type("application/rss+xml; charset=utf-8")
        .body(rss))
}

/// Atom Feed
pub async fn atom_xml(
    post_repo: web::Data<PostRepository>,
) -> Result<HttpResponse, actix_web::Error> {
    let posts = match post_repo.get_published_recent(20).await {
        Ok(p) => p,
        Err(e) => return Ok(HttpResponse::InternalServerError().body(format!("Error: {}", e))),
    };

    let now = Utc::now().to_rfc3339();
    let feed_id = format!("urn:id:{}", cuid2::create_id());

    // 预分配容量
    let mut atom = String::with_capacity(4096);
    atom.push_str(r#"<?xml version="1.0" encoding="UTF-8"?>
<feed xmlns="http://www.w3.org/2005/Atom">
  <title>我的博客</title>
  <link href=""#);
    atom.push_str(BASE_URL);
    atom.push_str(r#"/" />
  <link href=""#);
    atom.push_str(BASE_URL);
    atom.push_str(r#"/atom.xml" rel="self" />
  <id>"#);
    atom.push_str(&feed_id);
    atom.push_str(r#"</id>
  <updated>"#);
    atom.push_str(&now);
    atom.push_str(r#"</updated>
  <author>
    <name>我的博客</name>
  </author>
  <subtitle>分享技术、生活和思考的博客</subtitle>
"#);

    for post in posts {
        let post_id = format!("urn:id:{}", post.id);
        let updated = post.updated_at.to_rfc3339();
        let published = post.published_at
            .map(|d| d.to_rfc3339())
            .unwrap_or_else(|| updated.clone());

        let summary = post.excerpt.clone()
            .unwrap_or_else(|| post.content.chars().take(200).collect::<String>() + "...");

        atom.push_str(&format!(r#"  <entry>
    <title>{}</title>
    <link href="{}/posts/{}" />
    <id>{}</id>
    <updated>{}</updated>
    <published>{}</published>
    <summary><![CDATA[{}]]></summary>
    <content type="html"><![CDATA[<p>{}</p><p><a href="{}/posts/{}">阅读全文</a></p>]]></content>
  </entry>
"#,
            escape_xml(&post.title),
            BASE_URL, post.slug,
            post_id,
            updated,
            published,
            summary,
            escape_xml(&post.content.chars().take(500).collect::<String>()),
            BASE_URL, post.slug
        ));
    }

    atom.push_str("</feed>");

    Ok(HttpResponse::Ok()
        .content_type("application/atom+xml; charset=utf-8")
        .body(atom))
}

fn escape_xml(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&apos;")
}
