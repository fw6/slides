---
theme: default
layout: center
highlighter: shiki
css: unocss
colorSchema: light
transition: fade-out
mdc: true
growX: 50
growY: 130
growSize: 1.5
favicon: /trip.ico
title: 'Git深入浅出'
titleTemplate: '%s - Slides'
---

<h1 flex="~ col">
<div text-2xl op80>Git深入浅出<devicon-git ml-1 /></div>
</h1>

<div uppercase text-sm tracking-widest op50>
UGC前端组 冯伟
</div>

<div abs-br mx-35 my-11 flex="~ col gap-4 items-end" text-left>
  <img src="/trip-group.png" h-7 alt="Trip.com Group">
  <div text-xs opacity-75 mt--4>2023/12/27</div>
</div>

<!--
维基百科将Git描述为一个分布式版本控制软件, 我们首先来了解下什么是版本控制系统
-->

---
layout: intro
---

## 什么是版本控制(VCS)

<div mt-4 />

> 版本控制是一种记录一个或若干文件内容变化，以便将来查阅特定版本修订情况的系统


<div text-gray-500>
<v-clicks>

- 本地版本控制系统 (如: [RCS](https://www.gnu.org/software/rcs/))
  - 可以复制整个项目目录来保存不同版本, 改名+备份时间以示区别
  - RCS工作原理是在硬盘上保存补丁集, 应用所有的补丁, 可以重新计算出各个版本的文件内容

- 集中化版本控制系统（CVCS，centralized version control systems） (如: CVS, Subversion, Perforce)
  - 单一的集中管理的服务器, 保存所有文件的修订版本
  - 协同工作的人们都通过客户端连到这台服务器, 取出最新的文件或者提交更新
  - 本地只有当前版本以及部分历史信息

- 分布式版本控制系统（DVCS，distributed version control system） (如: Git, Mercurial, Bazaar, Darcs)
  - 客户端把代码仓库完整地镜像下来, 包含完整的历史记录
  - 要和其他人分享变更内容时, 才需要连接网络

</v-clicks>
</div>


<!--
有了它，就能将特定文件回溯到之前的状态，甚至将整个项目都回退到过去某个时间点的状态

可以比较文件的变化细节，查出最后是谁修改了哪个地方，从而找出导致怪异问题出现的原因，又是谁在何时报告了某个功能缺陷等等。

使用版本控制系统通常还意味着，就算你乱来一气把整个项目中的文件改的改删的删，也照样可以轻松恢复到原先的样子。

版本控制系统分为3种:
1. 复制整个项目目录来保存不同版本，或许还会改名+备份时间以示区别。好处是简单，缺点是容易犯错。比较流行的一种方式是[RCS](https://www.gnu.org/software/rcs/)，工作原理是在硬盘上保存补丁集（补丁是指文件修订前后的变化），通过应用所有的补丁，可以重新计算出各个版本的文件内容。
2. 这类系统，诸如CVS、Subversion以及Perforce等，都有一个单一的集中管理的服务器，保存所有文件的修订版本，而协同工作的人们都通过客户端连到这台服务器，取出最新的文件或者提交更新。缺点是中央服务器的单点故障，如果宕机一小时，那么在这一小时内，谁都无法提交更新，也就无法协同工作。如果中央数据库所在的磁盘发生损坏，或者数据库备份没做好，就会导致丢失所有数据。
3. 这类系统，诸如Git、Mercurial、Bazaar以及Darcs等，客户端不只提取最新版本的文件快照，而是把代码仓库完整地镜像下来，包括完整的历史记录。

-->

---
layout: intro
growX: 80
growY: -10
---

## Git与其他版本控制系统的差别

最主要差别在于Git对待数据的方式

<v-clicks>

> 其他大部分系统以文件变更列表的方式存储信息，这类系统**将他们存储的信息看作一组基本文件和每个文件随时间逐步累积的差异**。他们通常被称作基于差异（delta-based）或增量（incremental）的版本控制系统。

<v-clicks relative>

<img of-hidden absolute :class="$clicks > 2 ? 'hidden' : ''" src="/delta_base.png" alt="CVCS 集中化版本控制系统图示" w-100 />

<div flex items-start gap-8>

<div>

**Git更像是一个小型的文件系统，提供了许多以此为基础构建的超强工具，而不是一个简单的VCS。**

1. 几乎所有操作都是本地执行
2. Git在数据存储前计算校验和来保证完整性
   > Git用于计算校验和的机制叫做SHA-1散列（hash，哈希）。这是一个由40个十六进制字符（0-9和a-f）组成的字符串，基于Git中文件的内容或目录结构计算出来。实际上，Git数据库中保存的信息都是以文件内容的哈希值来索引，而不是文件名。
3. Git一般只添加数据

</div>

![Alt text](/snipshot_base.png){.w-240}

</div>


</v-clicks>
</v-clicks>

<!--

Git不按照以上方式对待或保存数据。反之，Git更像是把数据看作是对小型文件系统的一组快照。

在Git中，每当你提交更新或保存项目状态时，**它基本上就会对当时的全部文件创建一个快照并保存这个快照的索引**。

为了高效，如果文件没有修改，Git不会再次保存该文件，只保留一个链接指向之前存储的文件。Git对待数据更像是一个**快照流**(写入时复制技术实现)。

1. 几乎所有操作都是本地执行。如：浏览项目历史、提交更新、查看分支等等。意味着离线情况下几乎可以进行任何操作。
2. Git保证完整性。Git中所有数据在存储前都计算校验和，然后以校验和来引用。这意味着不可能在Git不知情时更改任何文件内容或目录内容。
   > Git用于计算校验和的机制叫做SHA-1散列（hash，哈希）。这是一个由40个十六进制字符（0-9和a-f）组成的字符串，基于Git中文件的内容或目录结构计算出来。实际上，Git数据库中保存的信息都是以文件内容的哈希值来索引，而不是文件名。
3. Git一般只添加数据。几乎所有操作都是添加数据。你执行的大部分操作，只需向数据库中填入数据，而很少会做任何不可逆的操作，如删除数据。

-->

---
layout: default
growX: 80
growY: 10
---

## 🧪实验: 创建一条提交

<div mt-8 />

<v-clicks>

```shell
$ git commit -a -m 'Initial commit'
```

<div mt-8 text-center text-4xl>🙅</div>

<div mt-8>

- 底层指令: `git hash-object`、`git update-index`、`git write-tree`、`git commit-tree`
- 上层指令: `git commit`、`git add`、`git push`、`git pull`

</div>

</v-clicks>

<!--

我们以创建一次提交开始,来深入了解Git的工作原理
注意,为了了解Git的本质, 接下来我们不会直接使用`git commit`!

Git本质是一个内容寻址文件系统,并在此基础上提供了一个版本控制系统的用户界面

Git最初是一套面向版本控制系统的工具集,而不是完整的版本控制系统,所以它还包含了一部分用于完成底层工作的子命令, 一般称这部分命令为底层命令,而那些更友好的命令被称为上层命令.

底层命令得以让你窥探Git内部的工作机制,也有助于说明Git是如何运作的

演示: 创建空项目, 讲解.git目录结构

.git目录包含了几乎所有Git存储和操作的东西.如需备份或复制一个版本库,只需要复制这个目录即可

- config: 项目特有的配置选项
- info: 包含一个全局性排除文件,用以放置那些不希望被记录在.gitignore文件中的忽略模式
- hooks: 包含客户端或服务端的钩子脚本
- objects: 存储所有数据内容
- refs: 存储指向数据(分支、远程仓库、标签等)的提交对象的指针
- HEAD: 指向当前所在分支的指针
- index: 暂存区信息

Git的核心是一个简单的键值对数据库,可以通过键(内容的哈希值)来访问值(文件内容),这个数据库被称为对象数据库

-->

---
layout: center
growX: 0
growY: 70
---

### 创建数据对象

<div mt-4 />

1. `echo "some text" | git hash-object -w --stdin` 写入文件内容到Git数据库
2. `git cat-file -p <hash>` 查看对象内容(`-p` 选项指示自动判断文件类型)
3. `git hash-object -w <file>` 将文件内容写入对象数据库, 并返回该对象的哈希值
4. `git cat-file -t <hash>` 查看对象类型 (`blob`)
5. `tree .git/objects` 查看存入的数据

<!--

1. 创建一个空项目,并执行`git init`初始化
2. 演示空的`objects`目录
3. 通过`git hash-object`命令,将一个文件内容写入对象数据库,并返回该对象的哈希值`echo "some text" | git hash-object -w --stdin`
4. 上述命名输出40个字符的校验和,这个校验和是对象的名字
5. 此时查看objects目录,可以找到与新内容对应的文件,校验和前2个字符作为目录名,后38个字符作为文件名
6. 一旦将内容存储在对象数据库中, 可通过`git cat-file -p <hash>`查看对象内容(-p选项指示自动判断文件类型)
7. 接下来创建个新文件,并通过hash-object存入数据库中, 修改下内容,再次存入
8. 此时对象数据库记录了该文件的两个版本(存入的第一条数据还在, tree .git 查看)
9. 上述记录的为数据对象 (运行`git cat-file -t <hash>`查看), 记录文件的每一个版本并不现实,此外文件名也无法记录


-->

---
layout: center
growX: 10
growY: 80
---

### 创建树对象

<div mt-4 />

1. `git update-index --add --cacheinfo 100644 <hash> <file>` 将数据对象添加到暂存区
2. `git ls-files --stage` 查看暂存区内容
3. `git write-tree` 将暂存区内容写入对象数据库,并返回树对象的哈希值
4. `git cat-file -t <hash>` 查看对象类型 (`tree`)

![Alt text](/tree_object.png){.w-120}

<!--

接下来探讨树对象,它能解决 文件名保存问题,也能将多个文件组织到一起

Git使用类似unix文件系统的方式来存储树对象. 一个树对象包含1或多条数对象记录,每条记录含有一个指向数据对象或子树对象的指针,以及相应的模式、类型、文件名信息(随便找个项目, 通过`git cat-file -p main^{tree}`查看)

如上图,展示了简化版的树对象模型

通常, Git根据某一时刻暂存区(即Index区域)所表示的状态创建并记录一个对应的树对象,如此重复便可依次记录(某个时间段内)一系列树对象

为创建一个树对象,我们需要暂存一些文件到暂存区

演示:
1. 执行`git update-index --add --cacheinfo 100644 <hash> <file>`将数据对象添加到暂存区(100644表明是一个普通文件, 参考UNIX常见的文件模式)
2. 上述命令将文件的第一个版本保存到了暂存区, 通过`git ls-files --stage`查看暂存区内容
3. 接下来,通过`git write-tree`命令将暂存区内容写入对象数据库,并返回树对象的哈希值
4. 此时执行`git cat-file -t <hash>`查看对象类型,可以看到是一个树对象
5. 接下来我们再将文件的第二个版本添加到暂存区,并新增一个文件,并通过`git write-tree`命令将暂存区内容写入对象数据库,并返回树对象的哈希值
6. 执行`git cat-file -p <hash>`查看树对象内容,可以看到树对象记录了文件的最新版本,以及新增的文件

-->

---
layout: center
growX: 10
growY: 120
---

### 创建提交对象

<div mt-4 />

1. `git commit-tree <hash> -m 'Initial commit'` 创建提交对象,并返回提交对象的哈希值
2. `git log --stat <hash>` 查看提交对象内容

![Alt text](/commit_test_graph.png){.mt-8.w-160}

<!--

上一步,我们得到了2个树对象, 分别代表我们想要跟踪的不同项目的快照

然而要重用这些快照,我们必须记住上述两个树对象的哈希值, 并且完全不知道谁保存了快照,在什么时刻保存了这些快照,以及为什么保存快照

为了解决这个问题, Git引入了提交对象

演示:
1. 执行`git commit-tree <hash> -m 'Initial commit'`命令,创建一个提交对象,并返回提交对象的哈希值
2. 此时执行`git cat-file -p <hash>`查看提交对象内容,可以看到提交对象记录了树对象的哈希值,以及提交信息
3. 接下来我们创建第二个提交对象`git commit-tree <hash> -p <prev hash> -m 'Second commit'`
4. 此时执行`git log --stat <hash>`, 可查看到我们先前的两次提交

以上就是我们运行`git add`+`git commit`命令时,Git所做的工作:
> 将被改写的数据保存为数据对象、更新暂存区、记录树对象,最后创建一个指明顶层树对象和父提交的提交对象

这三种主要的Git对象——数据对象、树对象和提交对象, 最初都以单独文件的格式保存在`.git/objects`目录下(执行`tree .git/objects`查看)

最终这两次提交的关系如图所示

-->

---
layout: intro
growX: 120
growY: 20
---

### `references`引用

<div mt-5 />

<v-click>

```shell
$ echo ffda080b84b4a640d811a9d653151675dc76ea33 > .git/refs/heads/master
```

> ⚠️ ‼️: 需要完整的SHA-1值

<div pt-4 />

```shell
$ git update-ref refs/heads/master 4e5825
```

</v-click>

<v-click>

![Alt text](/commit_test_refs_branch.png){.mt-8.h-60}

</v-click>

<!--

我们前面创建了2次提交,但是每次查看都需要输入提交对象的哈希值,这样很不方便

如果有个文件能保存SHA-1值,而该文件有个简单的名字,然后用这个名字替代原始的SHA-1值,那就太方便了.

在Git中,这种简单的名字被称为引用(Reference),它们存储在`.git/refs`目录下

创建一个引用,使用这两个命令都可以,但不提倡第一种那样直接编辑引用文件
此时执行`git log <branch>`与执行`git log <hash>`效果是一致的.

而这就是Git分支的本质了, 一个指向某一系列提交之首的指针或引用.
当运行`git branch <branch>`时, Git实际上会执行`git update-ref`命令, 取得当前所在分支的最新提交对应的SHA-1值并将其加入到你要创建的任何引用中
这里其实还有个坑, ,当执行`git branch <branch>`时Git如何知道最新提交的SHA-1值呢? 这里就要提到HEAD指针了, 我们先暂时不讲,后面会详细介绍

接下来,我们继续探究Git分支原理

-->

---
layout: default
growX: 100
growY: 10
---

## Git分支原理

> Git保存的不是文件的差异或变化,而是一系列不同时刻的**快照**

<div flex items-center mt-8>
<v-clicks>

![Alt text](/branch_deepin.png){.h-50}

<div pl-4 self-stretch>

![Alt text](/commit_object.png){.w-120}

</div>

</v-clicks>

</div>
<v-click>

**Git的分支,本质上仅仅是指向提交对象的可变指针**

</v-click>

<!--

Git的分支模型被称为它的必杀技特性. 其处理分支的方式可谓是难以置信的轻量,创建新分支几乎在瞬间完成, 并且进行分支切换也是一样便捷.

回忆下前面演示的, Git是他如何保存数据的
当使用`git commit`时,Git会先计算每一个子目录的校验和,然后在Git仓库中将这些校验和保存为树对象,Git会创建一个提交对象,包含指向树对象的指针.

Git仓库中有这几个对象: Blob对象(保存文件快照)、一个树对象(记录目录结构和blob对象索引)以及一个提交对象(包含指向前述树对象的指针和所有提交信息)

而Git的分支, 本质上仅仅是指向提交对象的可变指针.

也就是在`.git/refs/heads`目录下的文件, 都是该仓库的本地分支
既然有本地分支, 那么肯定还有远程分支, 远程分支保存在`.git/refs/remotes`目录下

-->

---
layout: two-cols-header
growX: 80
growY: 10
---

## 远程分支

::left::

![Alt text](/remote_branch.png){.w-100}

::right::

![Alt text](/forked_remote_branch.png){.w-100}

<!--

Git中的远程跟踪分支是远程分支状态的引用, 它们是你无法移动的本地引用. 一旦进行网络通信,Git会自动地更新远程跟踪分支的位置.

可以把他们类比书签🔖, 可以提醒你该分支在远程仓库中的位置就是你最后一次连接到他们的位置

> 使用`git branch -a`列出所有的远程分支与本地分支

远程分支以`<remote>/<branch>`

要与给定的远程仓库同步数据,运行`git fetch <remote>`命令,这个命令查找`origin`是哪个服务器并从中抓取本地没有的数据,更新本地数据库,移动`origin/<branch name>`指针到更新之后的位置

每次从远程仓库抓取,本地不会自动生成一份可编辑副本,只有一个不可修改的`origin/server_issue233`指针
-->

---
layout: intro
growX: 80
growY: 50
---

## 跟踪分支

跟踪分支是与远程分支有直接关系的本地分支

<v-clicks>

```shell
$ git checkout -b issue233 origin/issue233
$ git checkout --track origin/issue233
```

```shell
$ git branch -u origin/server_issue233
```

```shell
$ git merge @{u}
$ git rebase @{u}
```

```shell
$ git branch -vv
* develop 0997a9f [origin/develop: behind 147] fix: 确认跳转发布页参数
  release 65304cb [origin/release] Merge branch 'feat233' into 'release'
```

</v-clicks>

<!--


从一个远程跟踪分支检出一个本地分支会自动创建所谓的“跟踪分支”(它跟踪的分支被称为上游分支).

你可以在任意时间修改跟踪的上游分支

设置好上游分支后,可以通过简写`@{upstream}`或`@{u}`来引用上游分支.

要查看所有分支的跟踪分支,可以运行`git branch -vv`


可以看到正在跟踪的远程分支是否领先/落后

> 这些数字的值并非准确,取决于你上次抓取的数据`git fetch`.
> 这个命令并没有联网,只是告诉你关于本地缓存的服务器数据

-->

---
layout: intro
clicks: 1
growX: 20
growY: 10
---

## 拉取

<div>

<code>git pull</code><span :class="[$clicks > 0 ? '' : 'hidden']"> = <code>git fetch</code> + <code>git merge</code></span>

</div>

```
	  A---B---C master on origin
	 /
D---E---F---G master
	^
	你仓库中的 origin/master
```

<div mt-4 />

```
	  A---B---C origin/master
	 /         \
D---E---F---G---H master
```

<!--

使用`git fetch`从服务器抓取本地没有的内容时,它并不会修改工作目录中的内容
它只会获取数据告诉你本地分支落后远程分支多少提交

如果你想要自动合并远程分支到当前分支,可以使用`git pull`命令
使用`git pull`进行拉取操作,可以理解为`git fetch`+`git merge`

如果当前是一个跟踪分支,`git pull`会查找当前分支跟踪的服务器与分支,从服务器抓取数据然后尝试快速合并入远程分支.
如果有冲突,可以指定`--rebase`选项,这样Git会用你的提交覆盖远程分支,而不是用远程分支的提交覆盖你的提交

假设当前分支在master, 执行`git pull`获取远程分支并重放与本地分支有分歧(E)以来的变化,最后将本地分支指向最新的提交(H),它是一个合并提交,因为它有两个父提交

你也可以在pull的时候指定`--rebase`选项,指定调和分歧的方式

警告: 最好在拉取之前将本地修改弄到工作状态,或使用git stash命令暂存

-->

---
layout: two-cols-header
growX: -10
growY: 100
---

## 变基 vs. 合并

<div my-2 />

```
	  A---B---C feat1
	 /
D---E---F---G master
```

::left::

### 变基

```
            A'--B'--C' topic
            /
D---E---F---G master
```

::right::

<div ml-2>

### 合并

```
	  A---B---C feat1
	 /         \
D---E---F---G---H master
```

</div>

<!--

接下开我们来探讨下rebase与merge的区别
在Git中有两种整合来自不同分支的修改方法: merge与rebase

变基会提取A、B、C中引入的提交和修改,然后在F、G的基础上应用一次
> 使用`rebase`命令将提交到某一分支的所有修改都移到另一分支中,就好像“重新播放”一样
> 原理是首先找到这两个分支的最近共同祖先,然后对比当前分支相对于该祖先的历史提交,提取相应的修改并存为临时文件,然后将当前分支指向目标基底,最后以此将之前另存为临时文件的修改依序应用.

合并会把会把两个分支的最新快照(C、G)以及二者的共同祖先(E)进行3方合并, 合并产生新的快照并提交

这两种整合的方法最终结果没任何区别,但是变基的历史提交更为整洁.在查看经过变基的历史记录时可以看到,尽管实际开发工作是并行的,但他们看上去就像是串行的一样,提交历史是一条直线没有分叉

-->

---
layout: intro
growX: -10
growY: 80
---

### 变基的风险

变基操作的实质是丢弃一些现有的提交,然后相应地新建一些内容一样但实际上不同的提交

```
  A--- feat1
 /
D---E--- master
```

userA (rebase)
```
      A’--- feat1
     /
D---E--- master
```

userB (HEAD)
```
      A---B--- feat1
     /
D---E--- master
     \
      A’--- origin/feat1
```

<!--
演示变基操作的风险

1. admin在main初始提交并推送
2. userA创建feat1分支,修改并推送
3. userB在feat1分支,修改并提交
4. admin在main分支,修改并提交
5. userA在feat1分支,`git rebase main`变基并强制推送
6. userB此时在feat1分支,处境尴尬😅, 如果`git pull`会进行一次合并,生成新的提交内容(演示git pull)
7. 此时查看会看到被变基的内容被找了回来!

有人推送了基于变基的提交并丢弃了你本地开发所基于的一些修改 此时可以使用变基解决变基, git将会:
> 变基时`A`和`A'`几乎一样, 此时再次变基将会合并这两个提交,并且不会丢失`B`提交

-->

---
layout: fact
growX: 50
growY: 50
---

# 变基 vs. 合并

<p pt-4 px-4>只对尚未推送或分享给别人的本地修改执行变基操作清理历史，从不对已推送至别处的提交执行变基操作</p>

<!--

我们已经了解了变基的风险, 那什么情况下适合使用变基呢?

-->

---
layout: intro
growX: 80
growY: 80
---

## cherry-pick / rebase / revert

- cherry-pick: 获得在单个提交中引入的变更，然后尝试将作为一个新的提交引入到当前分支上
- rebase: 自动化的 cherry-pick 命令, 计算出一系列的提交，然后再以它们在其他地方以同样的顺序一个一个的 cherry-picks 出它们。
- revert: 本质上就是一个逆向的 git cherry-pick 操作, 将提交中的变更以完全相反的方式应用到一个新创建的提交中，本质上就是撤销或者倒转。

---
layout: intro
clicks: 3
growX: 100
growY: 100
---

## 重置

| 树       | 用途                            |
| -------- | ------------------------------- |
| HEAD     | 上次提交的快照,下次提交的父节点 |
| 索引     | 预期的下次提交快照              |
| 工作目录 | 沙盒                            |

<div mt-4 flex algin-start text-center gap-8 :class="[!$clicks && 'invisible']">
<div w-50>
    <div bg-gray-200 px-2 py-1>HEAD</div>
    <div pt-4 border border-t-0 border-black>
        <div mx-4 mb-4 px-4 py-2 rounded-24 bg-sky-200 :class="[$clicks < 3 && 'invisible']">
            file.txt v1
        </div>
    </div>
    <div mt-2 :class="[$clicks < 3 && 'invisible']"><code>git commit</code></div>
</div>

<div w-50>
    <div bg-cyan-600 px-2 py-1 text-white>Index</div>
    <div pt-4 border border-t-0 border-black>
        <div mx-4 mb-4 px-4 py-2 rounded-24 bg-sky-200 :class="[$clicks < 2 && 'invisible']">
            file.txt v1
        </div>
    </div>
    <div mt-2 :class="[$clicks < 2 && 'invisible']"><code>git add</code></div>
</div>

<div w-50>
    <div bg-rose-600 px-2 py-1 text-white>Working Directory</div>
    <div pt-4 border border-t-0 border-black>
        <div mx-4 mb-4 px-4 py-2 rounded-24 bg-sky-200>
            file.txt v1
        </div>
    </div>
</div>
</div>

<!--

接下来我们将探讨Git的`reset`和`checkout`, 相信很多人对这两个命令很困惑, 它们的作用有很多重叠, 但是又有很多不同

要真正理解并恰当使用它们,就要按照Git的思维方式来思考. 这两个命令就是管理并操作Git的三棵树

`HEAD`这棵树, 不知大家有没有注意到! 在最开始的实验中,我们打印了`.git`的目录结构,可以看到有个`HEAD`文件, 这个文件就是`HEAD`树的指针, 它指向当前所在分支的指针, 也就是`refs/heads`目录下的文件
可以简单理解为: 该分支最后一次提交的快照.
> `git cat-file -p ^HEAD` 查看HEAD指向的提交
> `git ls-tree -r HEAD` 查看HEAD指向的提交包含的文件

索引树是预期的下次提交, 即Git的暂存区. 通过`git add`命令可以将工作目录中的文件添加到索引树中, `git commit`通过索引树中的文件创建新的提交
> `gut ls-files -s`查看索引文件

工作目录也被称为工作区, 它是Git的沙盒, 你可以在其中修改文件, 添加文件, 删除文件, 但是这些修改并不会影响索引树和HEAD树, 除非你执行`git add`命令, 将工作目录中的文件添加到索引树中

动画演示:
1. 切换分支或克隆时,HEAD指向新的分支引用, 此时只有工作目录有内容
2. 运行`git add`获取工作目录中的内容, 并将其复制到索引中
3. 运行`git commit`获取索引中的内容并保存为一个永久快照,然后创建一个指向该快照的提交对象,并将HEAD指向该提交对象(refs/heads/master的指向新的SHA-1值)
4. 此时三棵树已经完全相同

-->

---
layout: intro
growX: 100
growY: 20
---

## 重置

<div mt-4 />

<v-click>

- `--soft` 仅移动HEAD指针指向
- `--mixed` 移动HEAD指针和重置索引(取消暂存文件,与`git add`相反)
- `--hard` 移动HEAD指针、重置索引和重置工作目录

</v-click>

<v-click>

误 `reset` 后的数据恢复:

1. `git reflog` 引用日志
    ```shell
    $ git reflog
    $ git reset --hard HEAD@{1}
    ```

2. `git fsck` 检查数据库完整性
    ```shell
    $ git fsck --full
    $ git reset --hard HEAD@{1}
    ```

</v-click>

<!--
接下来我们以一些具体的操作来演示`git reset`实际的效果

演示

总结下reset的三种模式

-->

---
layout: fact
growX: 80
growY: 20
---

<div text-4xl>

`checkout` 与 `reset` 的区别

</div>


<div mt-6 mx-16 leading-8>

1. `git checkout <branch>` 与 `git reset --hard <branch>` 类似, 但是 `checkout` 对工作目录是安全的,它会通过检查来确保不会将以更改文件丢掉(先试着简单合), 而 `reset` 则会强制覆盖工作目录


2. `git reset master` , develop自身和master指向同一个提交; `git checkout master` , develop不会移动,HEAD自身会移动(指向master)

</div>

<!--

reset和checkout有如下重要区别

-->

---
layout: intro
growX: 50
growY: 20
---

### 撤销合并

<div mt-4 />

1. 移动分支 `git reset --hard <hash>`
2. 还原提交 `git revert -m 1 HEAD~`

<!--

讲到重置, 有一个常见的场景就不得不提, 那就是撤销合并提交.

常见的有如下两种方法.

第一种缺点是会重写历史, 在多人协作的项目中, 会造成很大的麻烦.
移动分支会丢失那些在合并之后提交的内容

第二种方式是创建一个新的提交, 该提交会撤回所有已经存在提交的所有更改. Git称这个操作为还原.
其中 `-m` (mainline) 选项指定要保留的父节点, 当提交时会有2个父节点, 一个是HEAD, 一个是合并的分支, 通过该选项可以指定要保留的父节点
这个操作也是有风险的, 合并前与合并前公共提交节点间的所有提交都会丢失, 除非再次还原那次还原提交.
此外, 在合并之前需要确保被还原分支、合并分支与其两者的公共分支之前保持同步(即公共的父节点保持一致), 否则这部分提交也会丢失

-->

---
layout: intro
growX: 10
growY: 20
---

## 重写历史

<div mt-4 />

- 重写最后一次提交
- 修改多个提交信息

<!--

Git可以在本地随便重写历史, 接下来了解下重写历史的一些操作

-->

---
layout: intro
growX: 10
growY: 50
---

### 重写最后一次提交

<div mt-4 />

- 修改上次的提交信息
- 添加遗漏的文件
- 修改上次提交的文件

<div mt-3 />

```shell
$ git commit --amend
```

<!--

一般常见有: 修改上次的提交信息、添加遗漏的文件、修改上次提交的文件

可在执行`git commit`时增加`--amend`选项 通过创建一个新的提交替换上一次提交

-->

---
layout: intro
growX: 10
growY: 80
---

### 修改多个提交信息

<div mt-4 />

```shell
$ git rebase -i HEAD~3
```

<div mt-4 />

- `edit` 修改提交信息
- `reword` 仅修改提交信息
- `squash` 与前一个提交合并
- `fixup` 与前一个提交合并, 但丢弃提交信息

<!--

Git没提供改变历史的工具, 但可以使用变基工具来变基一系列提交

通过交互式变基, 可以在任何想要修改的提交后停止, 然后修改信息、添加文件或做任何想做的事

1. 新建仓库, 并创建3个提交
2. 执行`git rebase -i HEAD~3`命令, 会打开一个编辑器, 其中包含了3个提交的信息
3. 展示顺序与`git log`相反, 因为变基需要从远至近重放提交
4. 可以编辑对应commit前的指令,做一系列操作
5. 提交顺序也可以重新排序或移除
6. 也可以通过一些技巧, 实现将一个提交拆分为多个
   1. 使用`edit`指令修改提交信息
   2. 执行`git reset HEAD^`命令, 撤销此次提交并将修改从暂存区移出
   3. 多次执行`git add`、`git commit`命令, 将修改分为多个提交
   4. 拆分完成后, 执行`git rebase --continue`命令, 继续变基操作

-->

---
layout: intro
growX: 40
growY: 100
---

### 批量改写历史提交 [⚠️](https://git-scm.com/docs/git-filter-branch/zh_HANS-CN#_warning)


- 误提交了涉密内容、大文件 (alternative: [BFG Repo-Cleaner](https://rtyley.github.io/bfg-repo-cleaner/))
    ```shell
    $ git filter-branch --tree-filter 'rm -f passwords.txt' HEAD
    ```

- 修改提交人、作者的姓名、邮箱
    ```shell
    $ git filter-branch --commit-filter '
            if [ "$GIT_AUTHOR_EMAIL" = "mekhi@work" ];
            then
                    GIT_AUTHOR_NAME="Mekhi El";
                    GIT_AUTHOR_EMAIL="mekhi@personal.com";
                    git commit-tree "$@";
            else
                    git commit-tree "$@";
            fi' -- --all
    ```

- 将子目录独立成单独的仓库
    ```shell
    $ git filter-branch --subdirectory-filter apps/miniapp HEAD
    ```

<!--
如果需要修改多个历史提交信息, 可以使用`git filter-branch`命令
它会重写历史中所有提交, 并将其应用到新的提交中
-->

---
layout: intro
growX: 50
growY: 120
---

## submodule 与 subtree

<div mt-4 />

- submodule
  - 仓库中包含另一个仓库, 子模块是独立的(有自己的`.git`目录)
  - 在子模块目录可以: pull、add、commit、push
  - 可多层嵌套
  - clone时需要指定`--recursive-submodules`选项 (可配置`submodule.recurse`)
- subtree
  - 仓库中包含另一个仓库的快照 (`git subtree add`时会创建一条提交记录, 记录SHA-1值)
  - 和仓库其他目录没区别, 在subtree目录下的改动会被Git检测到
  - 允许单独push、pull到子项目

<div flex content-between gap-4>

```shell
$ git submodule add <url> <path>
$ git submodule update --init --recursive
$ git submodule update --remote --rebase
```

```shell
$ git subtree add --prefix=<prefix> <repository> <ref>
$ git subtree pull --prefix=<prefix> <repository> <ref>
$ git subtree push --prefix=<prefix> <repository> <ref>
```

</div>

[子模块示例](https://github.com/fw6/blog/tree/master/src/content)

<!--

Git提供了两种方式来管理子仓库: submodule与subtree

submodule: 仓库中包含另一个仓库, 但是它们是独立的, 仓库A包含仓库B, 但是仓库B的修改不会影响仓库A, 仓库A的修改也不会影响仓库B

subtree: 仓库中包含另一个仓库的快照, 仓库A包含仓库B的快照, 仓库B的修改不会影响仓库A, 仓库A的修改也不会影响仓库B

-->

---
layout: intro
growX: 50
growY: 70
---

## Git工具

<div mt-4 />

> 提供给Git的SHA-1字符数量不少于4个且无歧义, 即可获得此次提交

<div mt-4 />

- 查看引用日志: `git reflog`
- 使用祖先引用: `^` (脱字符), `~` (波浪号), 例: `git reset HEAD~3^2`
- 提交区间
  - `git log master..develop` : 在develop而不在master上的提交
  - `git log develop HEAD ^master` : 在当前分支和develop不在master上的提交
  - `git log master...develop --left-right --oneline` : 被其中之一包含但不被同时包含的提交
- 贮藏与清理: `git stash apply`, `git stash pop`, `git stash drop`
- 清理工作目录: `git clean -f -d` 从工作目录中移除未跟踪的文件和目录, `-x`选项会移除被忽略的文件

<!--

Git提供了很多使用的工具, 这里我们简单介绍几个

在你工作时Git会在后台保存一个引用日志, 它包含了最近几个月你的HEAD和分支引用指向的历史
每当HEAD指向的位置发生变化, Git就会将这个信息存储到引用日志中, 它可以配合其他命令使用
需要注意, 引用日志只存在于本地

需要指定引用的上一个提交, 可在引用后加如上两个 符号, ^后面可以加数字, 但需注意只能用在合并提交上,因为这表示第几个父提交
~后面可以加数字, 表示第几代父提交, 如果加数字, 则表示第几个父提交

这两个指令可以组合, 如`git reset HEAD~3^2` 表示引用的第三个父提交的第二父提交

当我们在查看Git日志时 需要过滤某个区间的提交, 这时可以使用提交区间语法.
- `master..develop`: 表示develop分支上还有哪些提交没合入master分支

贮藏会处理工作目录脏的状态(即跟踪文件和暂存的改动)保存到栈上, 你在任何时候都可以重新应用这些改动.
你也可以在贮藏时通过`-u`选项将未跟踪的文件也贮藏起来


-->

---
layout: intro
growX: 40
growY: 90
---

## 调试

<div mt-4 />

- `git bisect` 二分查找提交历史
  1. `git bisect start`
  2. `git bisect bad` 标记当前提交为坏提交
  3. `git bisect good <hash>` 标记一个已知的好提交
  4. Git检出中间提交, 使用`git bisect <goog | bad>`标记为好或坏
  5. `git bisect reset` 退出二分查找
- `git blame` 文件标注, 查看文件的每一行是谁修改的
- `git grep` 从提交历史、工作目录、索引中查找文本、正则表达式

<!--
Git 提供了一些工具来帮助你调试
 -->

---
layout: intro
growX: 20
growY: 100
---

## 配置 Git

<div mt-4 />

- 配置文件
  - `/etc/gitconfig` : 系统级配置文件
  - `~/.gitconfig` 或 `~/.config/git/config` : 用户级配置文件, 对应 `git config --global`
  - `.git/config` : 仓库配置文件, 对应`git config --local`
- 属性: [.gitattributes](https://github.com/gitattributes/gitattributes)
  - `*.docx diff=word` 匹配`.docx`文件使用`word`过滤器(将word文档转为可读文本文件, 便于查看差异), 增加配置 `git config diff.word.textconv docx2txt`
  - `*.pbxproj binary` 将所有该文件作为二进制文件处理, 不进行差异比较
- 忽略文件: [.gitignore](https://github.com/github/gitignore)
- [环境变量](https://git-scm.com/book/en/v2/Git-Internals-Environment-Variables)
- hooks
  - 客户端钩子: 提交工作流钩子、电子邮件工作流钩子、其他钩子
  - 服务器端钩子: pre-receive、update、post-receive

<!--

Git的配置文件有三个, 优先级从高到低依次为: `.git/config`、`~/.gitconfig`、`/etc/gitconfig`

Git配置文件是纯文本类型, 可以直接编辑, 也可以通过`git config`命令进行配置

Git属性, 是基于路径的配置项, 可以针对特定路径设置某些设置项, 例如: 指定某些文件使用特定的diff工具, 或者指定某些文件使用特定的换行符

-->

---
layout: end
---
