---
title: 'Introducing bon'
date: 2024-07-14
author: Veetaha
image: /bon-logo-medium.png
---

## My heading one

My text

as
asdas



<!-- more -->

[link](thisdot.co)

---

<VPTeamMembers size="small" :members="members" />

<script setup>
import { VPTeamMembers } from 'vitepress/theme'

const members = [
  {
    avatar: 'https://www.github.com/Veetaha.png',
    name: 'Veetaha',
    title: 'Lead developer',
    org: "elastio",
    orgLink: "https://github.com/elastio",
    desc: "Creator of bon",
    links: [
      { icon: 'github', link: 'https://github.com/Veetaha' },
    ]
  },
]
</script>
