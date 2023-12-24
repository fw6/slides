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
  <div text-xs opacity-75 mt--4>2023/12/28</div>
</div>

<!--
维基百科将Git描述为一个分布式版本控制软件, 我们首先来了解下什么是版本控制系统
-->

---
layout: intro
---

## 什么是版本控制(VCS)

版本控制是一种记录一个或若干文件内容变化，以便将来查阅特定版本修订情况的系统


<div text-gray-500>
<v-clicks>

- 本地版本控制系统
- 集中化版本控制系统（CVCS，centralized version control systems）
- 分布式版本控制系统（DVCS，distributed version control system）

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

**Git更像是一个小型的文件系统，提供了许多以此为基础构建的超强工具，而不是一个简单的VCS。**

![Alt text](/snipshot_base.png){.w-160}

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
layout: two-cols-header
growX: 120
growY: 20
---

### `references`引用

::left::

<v-click>

```shell
$ echo 4e5825 > .git/refs/heads/master
```

```shell
$ git update-ref refs/heads/master 4e5825
```

</v-click>

::right::

<v-click>

![Alt text](/commit_test_refs_branch.png){.ml-8.h-60.relative.top--24}

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
layout: default
---

<div v-click-hide :class="$clicks > 0 ? 'hidden' : ''">

![Alt text](/git_history_common.png){.w-120.mt-2}

</div>

<div :class="[$clicks > 0 ? '' : 'hidden']">

```shell
$ git branch testing
```

<v-click>

![Alt text](/new_branch.png){.w-120}

</v-click>

<v-click>

```shell
$ git log --oneline --decorate
```

</v-click>

</div>

<!--

一般分支历史如图所示,这里有个`HEAD`, 它是个特殊指针,用于确定当前所处的分支,可以将HEAD想象为当前分支的别名,如图所示,HEAD指向master分支

**演示 1-branch仓库示例**

Git创建分支, 会在当前提交对象上创建一个指针: 如图所示
可以使用如下指令,查看各个分支当前说指向的提交对象

**演示: main分支进行一次修改并提交**
这个项目的提交历史已经产生了分叉,上面两次提交发生在不同分支

你可以使用`git log`查看分叉历史:
```shell
$ git log --oneline --decorate --graph --all
```

-->

---
layout: two-cols-header
---

## 远程分支

::left::

![Alt text](/remote_branch.png){.w-100}

::right::

![Alt text](/forked_remote_branch.png){.w-100}

<!--

Git中的远程跟踪分支是远程分支状态的引用, 它们是你无法移动的本地引用. 一旦进行网络通信,Git会自动地更新远程跟踪分支的位置.

可以把他们类比书签🔖, 可以提醒你该分支在远程仓库中的位置就是你最后一次连接到他们的位置

我们可以使用`git branch -a`列出所有的远程分支与本地分支

远程分支以`<remote>/<branch>`

要与给定的远程仓库同步数据,运行`git fetch <remote>`命令,这个命令查找`origin`是哪个服务器并从中抓取本地没有的数据,更新本地数据库,移动`origin/<branch name>`指针到更新之后的位置

每次从远程仓库抓取,本地不会自动生成一份可编辑副本,只有一个不可修改的`origin/server_issue233`指针
-->

---
layout: intro
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
---

## 拉取

`git pull` = `git fetch` + `git merge`

<!--

使用`git fetch`从服务器抓取本地没有的内容时,它并不会修改工作目录中的内容
它只会获取数据告诉你本地分支落后远程分支多少提交

如果你想要自动合并远程分支到当前分支,可以使用`git pull`命令
使用`git pull`进行拉取操作,可以理解为`git fetch`+`git merge`

如果当前是一个跟踪分支,`git pull`会查找当前分支跟踪的服务器与分支,从服务器抓取数据然后尝试合并入远程分支.

-->

---
layout: intro
growX: 100
growY: 120
---

## Git三种状态

Git有三种状态，你的文件可能处于其中之一：**已提交**（committed）、**已修改**（modified）和**已暂存**（staged）。

<v-clicks>
<div flex items-center>

- 已修改表示修改了文件，但还没保存到数据库中。
- 已暂存表示对已修改文件的当前版本做了标记，使之包含在下次提交的快照中。
- 已提交表示数据已经安全地保存在本地数据库中。

![Alt text](/git_status.png){.w-120}

</div>
</v-clicks>

<!--
我能在开始深入学习Git之前，先对Git工作区建立一个感性的认识
首先是Git内文件的三种状态,它与工作区、暂存区、版本库的关系如下图所示
- 工作区是对项目的某个版本独立提取出来的内容。这些从Git仓库的压缩数据库中提取出来的文件，放在磁盘上供你使用或修改。
- 暂存区是一个文件，保存了下次将提交的文件列表信息，一般在Git仓库目录中。Git术语称作‘索引’，不过一般说法还是叫暂存区。
- Git仓库是Git用来保存项目的元数据和对象数据库的地方。这是Git最重要的部分，从其他计算机克隆仓库时，拷贝的就是这里的数据。(其实就是.git隐藏目录)
-->

---
layout: two-cols-header
---

## 文件的状态📃

我们可以通过`git status`查看文件的状态

::left::

<div v-click mr-4>

```shell
$ git status
On branch main
Changes to be committed:
  (use "git restore --staged <file>..." to unstage)
        renamed:    LICENSE.txt -> LICENSE
        modified:   README.md
        new file:   README.txt
        new file:   untrackted_staging_file

Changes not staged for commit:
  (use "git add/rm <file>..." to update what will be committed)
  (use "git restore <file>..." to discard changes in working directory)
        deleted:    README.md
        deleted:    untrackted_staging_file

Untracked files:
  (use "git add <file>..." to include in what will be committed)
        untracked
        untrackted_staged_file
```

</div>

::right::

<v-click>

```shell
$ git status -s
R  LICENSE.txt -> LICENSE
MD README.md
A  README.txt
AD untrackted_staging_file
?? untracked
?? untrackted_staged_file
```

</v-click>
<v-click>

- `?` 未被追踪的
- `A`：添加到索引树中的(索引接下来在Git三棵树中会讲到)
- `M`：在索引中被更新
- `D` 在索引中删除
- `R` 在索引中被重新命名

</v-click>

<!--

与Git状态息息相关的时文件的状态,执行如下命令可以查看文件的状态,可以看到当前工作区是干净的,没有任何修改,这里有个`working tree`,是Git的三棵树之一,后面会详细介绍

**打开代码仓库演示下状态!**

也可以通过`git status -s`查看文件的状态,这里的`-s`是`--short`的简写,可以看到文件的状态更加简洁
这里打印了部分状态,这里显示了一个双子母的状态代码,称为XY语法,XY的含义:
- `?` 未被追踪的
- `A`：添加到索引树中的(索引接下来在Git三棵树中会讲到)
- `M`：在索引中被更新
- `D` 在索引中删除
- `R` 在索引中被重新命名
-->

---
layout: intro
growX: 0
growY: 100
---

## Git的三棵树🌲

<div mt-8 />

| 树       | 用途                            |
| -------- | ------------------------------- |
| HEAD     | 上次提交的快照,下次提交的父节点 |
| 索引     | 预期的下次提交快照              |
| 工作目录 | 沙盒                            |

---
layout: center
---

HEAD 可简单理解为: 该分支最后一次提交的快照

> `git cat-file -p ^HEAD` 查看HEAD指向的提交
> `git ls-tree -r HEAD` 查看HEAD指向的提交包含的文件

---
layout: end
---
