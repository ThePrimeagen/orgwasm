use std::collections::HashSet;

use leptos::*;

#[component]
fn PaginationNavButton<T>(cx: Scope, name: &'static str, cb: T) -> impl IntoView
where
    T: Fn() + 'static,
{
    return view! {cx,
        <button on:click=move |_| cb()>
            {name}
        </button>
    };
}

#[derive(Clone)]
enum Bullet {
    Ellipse(usize),
    Idx(usize),
}

#[component]
fn PaginationBullet(cx: Scope, bullet: Bullet, signal: RwSignal<usize>) -> impl IntoView {
    if let Bullet::Ellipse(_) = bullet {
        return view! {cx,
            <><div class="bullet">{"..."}</div></>
        };
    } else if let Bullet::Idx(idx) = bullet {
        return view! {cx,
        <>
            <button on:click={move |_| {
                signal.set(idx);
            }}>
                {idx}
            </button>
        </>
        };
    }

    unreachable!("huh?");
}

#[component]
fn PaginationBulletList(cx: Scope, current: usize, count: usize, signal: RwSignal<usize>) -> impl IntoView {
    leptos::log!("current {}", current);

    let mut to_render = (0..3)
        .chain(current.saturating_sub(1)..=current + 1)
        .chain(count - 3..count)
        .filter(|x| *x < count)
        .collect::<HashSet<usize>>()
        .iter()
        .map(|x| *x)
        .collect::<Vec<usize>>();

    to_render.sort();

    let to_render = to_render
        .iter()
        .fold(vec![], |mut acc, curr| {
            if let Some(Bullet::Idx(last)) = acc.last() {
                if last + 1 != *curr {
                    acc.push(Bullet::Ellipse(acc.len()));
                }
            }

            acc.push(Bullet::Idx(*curr));

            return acc;
        });

    let (bullets, _) = create_signal(cx, to_render);

    return view! {cx,
        <For
            // a function that returns the items we're iterating over; a signal is fine
            each=bullets
            // a unique key for each item
            key=|counter| match counter {
                Bullet::Ellipse(x) => *x,
                Bullet::Idx(x) => *x
            }
            // renders each item to a view
            view=move |bullet: Bullet| {
                view! {
                    cx,
                    <PaginationBullet bullet=bullet signal=signal/>
                }
            }
        />
    };
}

#[component]
pub fn PaginationBar(cx: Scope, count: usize, signal: RwSignal<usize>) -> impl IntoView {
    return view! {cx,
        <div class="border w-full">
            <div class="flex">
                <PaginationNavButton name={"Prev"} cb=move || {
                    signal.update(|x| {
                        *x = x.saturating_sub(1);
                    });
                }/>
                {move || {
                    return view! {cx,
                        <PaginationBulletList current=signal.get() count=count signal=signal/>
                    };
                }}
                <PaginationNavButton name={"Next"} cb=move || {
                    signal.update(|x| {
                        if *x + 1 < count {
                            *x = *x + 1;
                        }
                    });
                }/>

            </div>
        </div>
    };
}
