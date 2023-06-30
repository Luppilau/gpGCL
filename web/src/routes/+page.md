---
page_title: Intro
title: Welcome to gpGCL
count: 25
list: ["this", "is", "dynamically", "generated", "using", "Svelte!"]
editor_value: |

 a := 1;
 b := 2;
 { b := a } [0.5] { a := b }
---

<svelte:head>

  <title>{page_title}</title>
</svelte:head>

<script lang="ts">
  import Card from "$lib/components/ui/Card.svelte"
  import Editor from "$lib/components/ui/Editor.svelte"
</script>

# { title }

### Capabilities

The markdown pages supports _most_ regular markdown **features**:

> Quotes
> — _gpGCL, June 2023_

| Tables | like  |
| ------ | ----- |
| this   | table |
| right  | here  |

Both `inline` code and

```javascript
console.log("code blocks");
```

Along with elements of web development:
<Card minHeight="200px">

#### Example editor

<Editor options={{readOnly: true}} value={editor_value} />
</Card>

<a href="/about">You can even create multiple pages and link between them</a>

And programatically generate elements:

<ul>
{#each list as item}
  <li>{item}</li>
{/each}
</ul>

and all the other good Svelte stuff.

### Concrete syntax

```
C ->  skip
      diverge
      tick(z)
      x:=e
      x:=P
      C;C
      {C} [] {C}
      {C} [p] {C}
      if (b) {C}
      if (b) {C} else {C}
      while (b) {C}

e ->  z
      x
      e * e
      e / e
      e % e
      e + e
      e - e
      e :- e

b ->  !b
      b && b
      b || b
      e < e
      e <= e
      e > e
      e >= e
      e == e
      e != e

P ->  normal(z,z)
      uniform(z,z)
      lognormal(z,z)
      exponential(z)

```
