use cosy_gameapi::Game;
use futures::StreamExt;

#[tokio::test]
async fn parallel_fetch_sets_logo() {
    let mut games = [
        Game {
            id: 1,
            name: "a".into(),
            logo_url: None,
            hero_url: None,
        },
        Game {
            id: 2,
            name: "b".into(),
            logo_url: None,
            hero_url: None,
        },
        Game {
            id: 3,
            name: "c".into(),
            logo_url: None,
            hero_url: None,
        },
    ];

    let include_logo = true;
    let include_hero = false;

    futures::stream::iter(games.iter_mut())
        .map(|game| async move {
            // simulate network delay per entry
            tokio::time::sleep(std::time::Duration::from_millis(5)).await;
            if include_logo {
                game.logo_url = Some(format!("https://example/{}/logo.png", game.id));
            }
            if include_hero {
                game.hero_url = Some(format!("https://example/{}/hero.png", game.id));
            }
        })
        .buffer_unordered(8)
        .for_each(|_| async {})
        .await;

    // all games should have logo set, none should have hero
    for g in games.iter() {
        assert!(g.logo_url.is_some());
        assert!(g.hero_url.is_none());
    }
}

#[tokio::test]
async fn parallel_fetch_sets_logo_and_hero() {
    let mut games = [
        Game {
            id: 10,
            name: "x".into(),
            logo_url: None,
            hero_url: None,
        },
        Game {
            id: 11,
            name: "y".into(),
            logo_url: None,
            hero_url: None,
        },
    ];

    let include_logo = true;
    let include_hero = true;

    futures::stream::iter(games.iter_mut())
        .map(|game| async move {
            tokio::time::sleep(std::time::Duration::from_millis(5)).await;
            if include_logo {
                game.logo_url = Some(format!("https://example/{}/logo.png", game.id));
            }
            if include_hero {
                game.hero_url = Some(format!("https://example/{}/hero.png", game.id));
            }
        })
        .buffer_unordered(8)
        .for_each(|_| async {})
        .await;

    for g in games.iter() {
        assert!(g.logo_url.is_some());
        assert!(g.hero_url.is_some());
    }
}
