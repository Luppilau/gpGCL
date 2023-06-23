---
page_title: Intro
title: Welcome to gpGCL
count: 25
list: [1, 2, 3, 4, "boo"]
---

<svelte:head>

  <title>{page_title}</title>
</svelte:head>

# { title }

### Short explanation of the framework

todo!()

### Markdown Capabilities

```javascript
let x = x;
```

Sometimes what you wrote last week is so good that you just _have_ to include it again.

I'm not gonna stand in the way of your egomania.

> Hello!
>
> — _Me, May 2019_

Yeah, thats right you can put wigdets in markdown (`.svx` files or otherwise). You can put markdown in widgets too.

Sometimes you need your widgets **inlined** because why shouldn't you.
Obviously you have access to values defined in YAML (namespaced under `metadata`) and anything defined in an fenced `js exec` block can be referenced directly.

Normal markdown stuff works too:

| like  | this |
| ----- | ---- |
| table | here |
| table | here |
| table | here |

And _this_ and **THIS**. And other stuff. You can also use all your favorite Svelte features, like `each` blocks:

<ul>
{#each list as item}
  <li>{item}</li>
{/each}
</ul>

and all the other good Svelte stuff.
