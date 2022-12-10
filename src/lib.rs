use mdbook::{
    book::Book,
    preprocess::{Preprocessor, PreprocessorContext},
    BookItem,
};
use pulldown_cmark::{CodeBlockKind, Event, Options, Parser, Tag};
use svgbob::{to_svg_with_settings, Settings};

pub struct Bob;

impl Bob {
    pub fn new() -> Bob {
        Bob
    }

    fn process_chapter(&self, content: &str) -> String {
        // 开始处理
        let settings = Settings {
            background: "transparent".into(),
            ..Settings::default()
        };

        // 检查 bob | svgbob

        // 临时存储bob内容
        let mut bob_content = String::new();
        // 标记可以开始新的内容
        let mut start_code_span = true;
        // 标记开始记录bob内容
        let mut in_bob = false;
        // 标记所在位置
        let mut code_span = 0..0;
        // 存储(片段，替代内容)
        let mut bob_blocks = vec![];

        let opts = Options::empty();

        let events = Parser::new_ext(content, opts);
        for (e, span) in events.into_offset_iter() {
            log::debug!("e={:?}, span={:?}", e, span);

            if let Event::Start(Tag::CodeBlock(CodeBlockKind::Fenced(code))) = e.clone() {
                if &*code == "bob" || &*code == "svgbob" {
                    // 标记开始
                    in_bob = true;
                    // 清空之前的内容
                    bob_content.clear();
                }
            }

            if !in_bob {
                continue;
            }

            if let Event::Text(_) = e {
                if start_code_span {
                    code_span = span;
                    start_code_span = false;
                } else {
                    code_span = code_span.start..span.end;
                }

                continue;
            }

            if let Event::End(Tag::CodeBlock(CodeBlockKind::Fenced(code))) = e {
                if &*code == "bob" || &*code == "svgbob" {
                    // 标记结束
                    in_bob = false;

                    let bob_content = &content[code_span.clone()];
                    let bob_svg = to_svg_with_settings(bob_content, &settings);
                    bob_blocks.push((span, bob_svg));

                    // 重新开始
                    start_code_span = true;
                }
            }
        }

        // 重新整理内容
        let mut content = content.to_string();
        // 这里倒序处理
        for (span, block) in bob_blocks.iter().rev() {
            content.replace_range(span.clone(), &block);
        }

        content
    }
}

impl Preprocessor for Bob {
    fn name(&self) -> &str {
        "svgbob"
    }

    fn run(&self, _ctx: &PreprocessorContext, mut book: Book) -> mdbook::errors::Result<Book> {
        book.for_each_mut(|item| {
            if let BookItem::Chapter(chapter) = item {
                chapter.content = self.process_chapter(&chapter.content);
            }
        });

        Ok(book)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn render_bob() {
        let content = r#"# Chapter

```bob
+-----+      .-----.
|  A  |----> |  B  |
+-----+      '-----'
```

Text
"#;
        let content_b = Bob::new().process_chapter(content);
        assert_eq!(content_b.contains("<svg xmlns"), true);
    }
}
