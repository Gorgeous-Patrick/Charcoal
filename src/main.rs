use charcoal::{cli, AppBuilder, Cli, Command, Speech, WordEntry};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    env_logger::init();

    match Cli::new() {
        Command::Query(args) => query_main(args).await,
        Command::Edit(args) => edit_main(args).await,
        Command::Clean => clean_main().await,
    }
}

async fn query_main(args: cli::QueryArgs) -> anyhow::Result<()> {
    let app_builder = AppBuilder::new();

    let mut config = app_builder.config()?;
    let cache = app_builder.cache()?;

    let word = args.query.to_owned();
    config.apply(args);

    let lang = whatlang::detect(word.as_ref())
        .expect("Language detection failed.")
        .lang();
    let word_speech = Speech::query(&word, &lang, &cache, config.speak);
    let word_query = WordEntry::query(&word, &lang, &cache).await?;

    if word_query.is_empty() {
        println!("Word not found.");
        return Ok(());
    }

    word_query.display(&word, &config);
    if let Err(err) = word_speech.await {
        log::error!("An error occured in speech module: {:?}.", err)
    }

    Ok(())
}

async fn edit_main(args: cli::EditArgs) -> anyhow::Result<()> {
    use std::process::Command;

    let editor = std::env!("EDITOR");
    let config_path = {
        let app_builder = AppBuilder::new();
        if args.reset {
            app_builder.config_fresh()?
        } else {
            app_builder.config()?
        }
        .path
    };

    let mut child = Command::new(editor).args([config_path]).spawn()?;
    child.wait()?;
    Ok(())
}

async fn clean_main() -> anyhow::Result<()> {
    let mut cache = AppBuilder::new().cache()?;
    let res = cache.clean()?;
    Ok(res)
}

/* TODO
 * 4. Authority
 */
