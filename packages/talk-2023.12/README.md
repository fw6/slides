# Git 基础

## 什么是版本控制

版本控制是一种记录一个或若干文件内容变化，以便将来查阅特定版本修订情况的系统。

有了它，就能将特定文件回溯到之前的状态，甚至将整个项目都回退到过去某个时间点的状态，可以比较文件的变化细节，查出最后是谁修改了哪个地方，从而找出导致怪异问题出现的原因，又是谁在何时报告了某个功能缺陷等等。
使用版本控制系统通常还意味着，就算你乱来一气把整个项目中的文件改的改删的删，也照样可以轻松恢复到原先的样子。

版本控制系统分为：
1. 本地版本控制系统：复制整个项目目录来保存不同版本，或许还会改名+备份时间以示区别。好处是简单，缺点是容易犯错。比较流行的一种方式是[RCS](https://www.gnu.org/software/rcs/)，工作原理是在硬盘上保存补丁集（补丁是指文件修订前后的变化），通过应用所有的补丁，可以重新计算出各个版本的文件内容。
2. 集中化的版本控制系统(CVCS，centralized version control systems)：这类系统，诸如CVS、Subversion以及Perforce等，都有一个单一的集中管理的服务器，保存所有文件的修订版本，而协同工作的人们都通过客户端连到这台服务器，取出最新的文件或者提交更新。缺点是中央服务器的单点故障，如果宕机一小时，那么在这一小时内，谁都无法提交更新，也就无法协同工作。如果中央数据库所在的磁盘发生损坏，或者数据库备份没做好，就会导致丢失所有数据。
3. 分布式版本控制系统（DVCS，distributed version control system）：这类系统，诸如Git、Mercurial、Bazaar以及Darcs等，客户端不只提取最新版本的文件快照，而是把代码仓库完整地镜像下来，包括完整的历史记录。

## Git 是什么

Git和其他版本控制系统最主要差别在于Git对待数据的方式。

其他大部分系统以文件变更列表的方式存储信息，这类系统**将他们存储的信息看作一组基本文件和每个文件随时间逐步累积的差异**。他们通常被称作基于差异（delta-based）或增量（incremental）的版本控制系统。
> 缺个图

Git不按照以上方式对待或保存数据。反之，Git更像是把数据看作是对小型文件系统的一组快照。
在Git中，每当你提交更新或保存项目状态时，**它基本上就会对当时的全部文件创建一个快照并保存这个快照的索引**。
为了高效，如果文件没有修改，Git不会再次保存该文件，只保留一个链接指向之前存储的文件。Git对待数据更像是一个**[快照](https://zh.wikipedia.org/wiki/%E5%BF%AB%E7%85%A7_(%E9%9B%BB%E8%85%A6%E5%84%B2%E5%AD%98))流**。
> 缺个图


Git更像是一个小型的文件系统，提供了许多以此为基础构建的超强工具，而不是一个简单的VCS。

1. 几乎所有操作都是本地执行。如：浏览项目历史、提交更新、查看分支等等。意味着离线情况下几乎可以进行任何操作。
2. Git保证完整性。Git中所有数据在存储前都计算校验和，然后以校验和来引用。这意味着不可能在Git不知情时更改任何文件内容或目录内容。
   > Git用于计算校验和的机制叫做SHA-1散列（hash，哈希）。这是一个由40个十六进制字符（0-9和a-f）组成的字符串，基于Git中文件的内容或目录结构计算出来。实际上，Git数据库中保存的信息都是以文件内容的哈希值来索引，而不是文件名。
3. Git一般只添加数据。几乎所有操作都是添加数据。你执行的大部分操作，只需向数据库中填入数据，而很少会做任何不可逆的操作，如删除数据。

## Git 的三种状态

Git有三种状态，你的文件可能处于其中之一：已提交（committed）、已修改（modified）和已暂存（staged）。
- 已修改表示修改了文件，但还没保存到数据库中。
- 已暂存表示对已修改文件的当前版本做了标记，使之包含在下次提交的快照中。
- 已提交表示数据已经安全地保存在本地数据库中。
> 缺个图。展示三种状态之间的关系

- 工作区是对项目的某个版本独立提取出来的内容。这些从Git仓库的压缩数据库中提取出来的文件，放在磁盘上供你使用或修改。
- 暂存区是一个文件，保存了下次将提交的文件列表信息，一般在Git仓库目录中。Git术语称作‘索引’，不过一般说法还是叫暂存区。
- Git仓库是Git用来保存项目的元数据和对象数据库的地方。这是Git最重要的部分，从其他计算机克隆仓库时，拷贝的就是这里的数据。

如果Git目录中保存着特定版本的文件，就属于*已提交*状态。如果文件已修改并放入了暂存区，就属于*已暂存*状态。如果自上次取出后，作了修改但还没有放到暂存区域，就是*已修改*状态。

## Git配置

Git自带一个git config的工具来帮助设置控制Git外观和行为的配置变量。这些变量存放在三个不同的位置：
1. /etc/gitconfig文件：包含系统上每一个用户及他们仓库的通用配置。如果使用带有--system选项的git config时，它会从此文件读写配置变量。
2. ~/.gitconfig文件：只针对当前用户。可以传递--global选项让Git读写此文件。
3. 当前使用仓库的Git目录中的config文件（就是.git/config）：针对该仓库。

每一个级别覆盖上一级别的配置，所以.git/config里的配置变量会覆盖/etc/gitconfig中的配置变量。

可以使用如下命令查看所有的配置以及它们所在的文件：
```shell
$ git config --list --show-origin
```

初次使用Git时要做的第一件事就是设置你的名字和邮箱地址。每次Git提交时都会引用这两条信息，如下：
```shell
$ git config --global user.name feng.w
$ git config --global user.email feng.w@trip.com
```

如果要检查你的配置，可以使用git config --list命令来列出所有Git当时能找到的配置。
```shell
$ git config --list
credential.helper=osxkeychain
init.defaultbranch=main
user.email=feng.w@trip.com
user.name=feng.w
```

可以查询某个特定配置项的原始值，它会告诉你哪个配置文件最后设置了该值：
```shell
$ git config --show-origin user.name
file:/Users/fengwei/.gitconfig-work     feng.w
```

## 仓库初始化

执行`git init`，将创建一个名为`.git`的子目录，这个子目录含有初始化的Git仓库中所有的必须文件，这些文件是Git仓库的骨干。此时项目中文件还未被跟踪


## 文件状态

检查文件状态：
```shell
$ git status
On branch master
Your branch is up-to-date with 'origin/master'.
nothing to commit, working directory clean
```

这说明当前的工作目录相当干净（已跟踪文件自上次提交后都未被更改过、也没有任何处于未跟踪状态的新文件）。
这里分支名是默认的`master`，你可以修改`init.defaultbranch`配置项来修改默认分支名。


未跟踪的文件意味着Git之前的快照（提交）中没有这些文件；Git不会自动将之纳入跟踪范围。这样的处理让你不必担心将生成的二进制文件或其他不想被Git跟踪的文件纳入仓库中。

要暂存更新，需要运行`git add`命令，可以用它开始跟踪新文件，或把已跟踪文件放到暂存区，还能用于合并时把冲突文件标记为已解决状态等。
可以将这个命令理解为：`精确地将内容添加到下一次提交中`

使用`git status`命令的输出十分详细，但用语略繁琐。可以使用`git status -s`或`git status --short`命令来查看简短的输出。
```shell
$ git status -s
 M README
MM Rakefile
A  lib/git.rb
M  lib/simplegit.rb
?? LICENSE.txt
```
> 来个示例

- `??`：新添加的未跟踪文件
- `A`：新添加到暂存区中的文件（已修改已暂存）
- `M`：修改过的文件（已修改未暂存）
- `MM`：修改过并放入暂存区的文件（已修改已暂存后，又新增了修改）

## 忽略文件

有些文件无需纳入Git管理，也不希望出现在未跟踪文件列表（通常是自动生成的文件，如日志文件，或编译期创建的临时文件）。可以创建一个名为`.gitignore`的文件，列出要忽略的文件模式。
```shell
$ cat .gitignore
*.[oa]
*~
```

第一行告诉Git忽略所有以`.o`或`.a`结尾的文件
第二行告诉Git忽略所有以波浪符`~`结尾的文件

文件`.gitignore`格式规范如下：
- 所有空行或者以`#`开头的行都会被Git忽略。
- 可以使用标准的`glob模式`匹配（递归应用到整个工作区）
- 匹配模式可以以`/`开头防止递归。
- 匹配模式可以以`/`结尾指定目录。
- 要忽略指定模式以外的文件或目录，可以在模式前加上叹号`!`取反。

所谓的[glob模式](https://en.wikipedia.org/wiki/Glob_(programming))，是一种简化的正则表达式。
- 星号`*`匹配零个或多个任意字符；
- `[abc]`匹配任何一个列在方括号中的字符（这个例子要么匹配一个a，要么匹配一个b，要么匹配一个c）；
- 问号`?`只匹配一个任意字符；
- 范围匹配，如`[0-9]`表示匹配所有0到9的数字；
- 使用两个星号`**`表示匹配任意中间目录，如`a/**/z`可以匹配`a/z`、`a/b/z`或`a/b/c/z`等。

再来看一个例子：
```shell
# 忽略所有的 .a 文件
*.a
# 但跟踪所有的 lib.a，即使你在前面忽略了 .a 文件
!lib.a
# 忽略doc目录下后缀名为md的子文件
doc/*.md
# 忽略doc目录下的所有pdf文件
doc/*.pdf
# 忽略任何路径为/ios/Pods的文件夹
**/ios/Pods/
# 忽略当前目录下的coverage文件
/coverage
# 忽略任何目录下名为.cxx的文件夹
.cxx/
```

> github有一个针对多种语言的十分详细的gitignore文件列表。可[在此](https://github.com/github/gitignore)找到它
> 一般一个项目只需一个`.gitignore`文件，它递归地应用到整个仓库。然而子目录也是可以有自己的`.gitignore`文件的。子目录中的`.gitignore`只作用于它所在的目录中（可以扩展/覆盖父级规则）。

## 查看修改

想知道具体修改了哪些地方，可以使用`git diff`命令。它比较的是工作目录中当前文件和暂存区域快照之间的差异，也就是修改之后还没有暂存起来的变化内容。
若要查看已暂存的将要添加到下次提交里的内容，可以用`git diff --staged`命令。这条命令将比对已暂存文件与最后一次提交的文件差异。

> `git diff`本身只显示尚未暂存的改动，而不是自上次提交以来所做的所有改动。

暂存区准备就绪之后，就可以提交了。运行提交命令：
```shell
$ git commit
```
会启动文本编辑器来输入提交说明.
> 启动的编辑器是通过SHELL的环境变量EDITOR指定的，一般为vim或emacs

也可以在命令后面加上`-m`参数，将提交说明直接跟在命令后面。

提交时记录的是放在暂存区的快照。任何还未暂存文件仍然保持已修改状态。每次运行提交命令都是对项目进行一次快照。

在提交时，给`git commit`加上-a选项，Git会自动把所有已经**跟踪过**的文件暂存起来一并提交，从而跳过`git add`步骤。

要从Git中移除某个文件，就必须要从已跟踪文件清单中移除（确切地说，是从暂存区域移除），然后提交。可以用`git rm`命令完成此项工作，并连带从工作目录中删除指定的文件，这样以后就不会出现在未跟踪文件清单中了。

希望将文件从Git仓库中删除，但仍然希望保留在当前工作目录中，可以使用`--cached`选项：
```shell
$ git rm --cached log/\*.log
```

## 移动文件

Git并不跟踪文件移动操作，只是在内部记录下文件移动的操作。所以，如果要移动文件，只需在Git中把它们重命名即可：
```shell
$ git mv file_from file_to
On branch master
Your branch is up-to-date with 'origin/master'.
Changes to be committed:
  (use "git reset HEAD <file>..." to unstage)

    renamed:    README.md -> README
```

其实，运行`git mv`就相当于运行了下面三条命令：
```shell
$ mv README.md README
$ git rm README.md
$ git add README
```

这样分开操作，Git也能知道是一次重命名操作，两者是一样的。

## 查看提交历史

查看提交历史，可以使用`git log`命令。不加任何参数的话，会按提交时间列出所有的更新，最近的更新排在最上面。

`git log`常用的选项是`-p`或`--patch`，它会显示每次提交引入的差异。也可以限制显示的日志数量，如`-2`表示只显示最近的两次更新。
```shell
$ git log -p -2
```
该选项除了显示基本信息之外，还附带了每次提交的变化。当进行代码审查，或者快速浏览某个搭档的提交所带来的变化时，这个参数就非常有用。

使用`git log`可以使用`--stat`查看提交的简略统计信息：
```shell
$ git log --stat
```

`--pretty`选项可以指定使用不同于默认格式的方式展示提交历史。比如，使用`oneline`参数，可以将每个提交放在一行显示，这样每个提交就只占一行了：
```shell
$ git log --pretty=oneline
```
其他还有：
- `short`
- `full`
- `fuller`
暂时的信息基本一致，但是详尽程度不一

`--pretty`选项还可以定制输出的格式。`format`参数可以指定输出的格式，然后指定相应的占位符。比如，`%h`表示提交对象（commit）的简短哈希值，`%an`表示作者名，`%ar`表示作者修订日期，`%s`表示提交说明。
```shell
$ git log --pretty=format:'%Cred%h%Creset -%C(yellow)%d%Creset %s %Cgreen(%cr) %C(bold blue)<%an>%Creset'
```

下表列出了常用的选项：
| 选项 | 说明                                        |
| ---- | ------------------------------------------- |
| %H   | 提交对象（commit）的完整哈希字串            |
| %h   | 提交对象的简短哈希字串                      |
| %T   | 树对象（tree）的完整哈希字串                |
| %t   | 树对象的简短哈希字串                        |
| %P   | 父对象（parent）的完整哈希字串              |
| %p   | 父对象的简短哈希字串                        |
| %an  | 作者（author）的名字                        |
| %ae  | 作者的电子邮件地址                          |
| %ad  | 作者修订日期（可以用 --date= 选项定制格式） |
| %ar  | 作者修订日期，按多久以前的方式显示          |
| %cn  | 提交者(committer)的名字                     |
| %ce  | 提交者的电子邮件地址                        |
| %cd  | 提交日期                                    |
| %cr  | 提交日期，按多久以前的方式显示              |
| %s   | 提交说明                                    |

不知你们有没有发现有作者还有个提交者。其实作者指的是实际做出修改的人，提交者指的是最后将此工作成果提交的人。例如，当你为某个项目发布补丁，某个核心成员将补丁并入了项目，那么这个核心成员就是提交者，而你则是作者。

当`oneline`和`format`选项与`--graph`选项配合使用时，可以看到分支合并图：
```shell
$ git log --pretty=format:'%h %s' --graph
*   bbbce290 (HEAD -> develop, origin/develop) Merge branch 'content/dev_s141_20231221' into develop
|\
| * 6a62441b (origin/content/dev_s141_20231221) fix: creation center change show label logic
* |   a36f6c07 Merge remote-tracking branch 'origin/kol/dev_s141_20231221' into develop
```

`--graph`选项会在左侧用ASCII字符绘制一幅表示分支合并历史的图形。

下表列出了`git log`常用的选项：
| 选项            | 说明                                                                                                  |
| --------------- | ----------------------------------------------------------------------------------------------------- |
| -p              | 按补丁格式显示每个更新之间的差异。                                                                    |
| --stat          | 显示每次更新的文件修改统计信息。                                                                      |
| --shortstat     | 只显示 -stat 中最后的行数修改添加移除统计。                                                           |
| --name-only     | 仅在提交信息后显示已修改的文件清单。                                                                  |
| --name-status   | 显示新增、修改、删除的文件清单。                                                                      |
| --abbrev-commit | 仅显示 SHA-1 的前几个字符，而非所有的 40 个字符。                                                     |
| --relative-date | 使用较短的相对时间显示（比如，“2 weeks ago”）。                                                       |
| --graph         | 显示 ASCII 图形表示的分支合并历史。                                                                   |
| --pretty        | 使用其他格式显示历史提交信息。可用的选项包括 oneline、short、full、fuller 和 format（后跟指定格式）。 |
| --oneline       | `--pretty=oneline --abbrev-commit` 的简写。                                                           |


如果觉得输出信息太多，可以使用`-n`选项限制显示的日志数量。不过在实际展示log时，Git默认会将所有输出传送到分页程序中，所以你一次只会看到一页的内容。
> 可以配置默认分页程序，如`git config --global core.pager 'delta'`

类似`--since`和`--until`这种按照时间做限制的选项很有用。
```shell
$ git log --since=2.weeks
```

这个命令可以是确定日期，也可以是相对日期。还可以过滤指定条件的提交。`--author`选项显示指定作者的提交，`--grep`选项搜索提交说明中的关键字。
另一个很有用的过滤器是`-S`选项，只显示添加或移除了某个关键字的提交。
```shell
$ git log -S function_name
```

下表列出了`git log`常用的过滤选项：
| 选项              | 说明                               |
| ----------------- | ---------------------------------- |
| -(n)              | 仅显示最近的 n 条提交              |
| --since, --after  | 仅显示指定时间之后的提交。         |
| --until, --before | 仅显示指定时间之前的提交。         |
| --author          | 仅显示指定作者相关的提交。         |
| --committer       | 仅显示指定提交者相关的提交。       |
| --grep            | 仅显示含指定关键字的提交           |
| -S                | 仅显示添加或移除了某个关键字的提交 |

下方命令只会显示最近两周内，feng.w提交的，包含`fix`关键字的提交(不包含合并提交)：
```shell
$ git log --since=2.weeks --author=feng.w --grep=fix --no-merges
```

## 撤销操作

在任何阶段，你都有可能想要撤销某些操作。注意，有些操作是不可逆的

有时候我们提交完才发现漏掉了几个文件没添加，或者提交信息写错了，此时可以运行带有`--amend`选项的提交命令尝试重新提交：
```shell
$ git commit --amend
```
这个命令会讲暂存区的文件提交。如果自上次提交以来还没做任何修改，那么快照保持不变，而你所修改的只是提交信息。
文本编辑器启动后，可以看到之前的提交信息。编辑后保存会覆盖原来的提交信息。
> 当你修补最后的提交时，与其说是修复旧提交，不如说是完全用一个**新的提交**替换旧的提交。

如果修改了两个文件，但都暂存了，可以使用`git reset HEAD <file>`来取消暂存特定文件，执行后该文件处于修改未暂存的状态。


如果不想保留对某个文件所做的修改,可以使用`git checkout -- <file>`来丢弃工作目录中的修改：
```shell
$ git checkout -- CONTRIBUTING.md
```
> 这是个危险的命令, 你对该文件所做的任何更改都会消失(使用最近提交的版本覆盖)

## 远程仓库

远程仓库指的是托管在因特网或其他网络中的项目仓库。

查看远程仓库:
```shell
$ git remote
origin
```

也可以使用指定选项`-v`查看远程仓库地址：
```shell
$ git remote -v
origin  git@git.dev.sh.ctripcorp.com:gs/gs-fe-kol-h5-nfes.git (fetch)
origin  git@git.dev.sh.ctripcorp.com:gs/gs-fe-kol-h5-nfes.git (push)
```
如果不只一个,会全部列出

添加远程仓库与拉取、抓取:
```shell
$ git remote add <remote> <url>
$ git fetch <remote>
$ git pull <remote> <branch>
```

`git fetch`命令会将数据拉取到本地仓库，执行完成后,你将拥有该远程仓库中所有分支的引用,可以随时合并或查看。

clone时会自动将其添加到远程仓库并默认以`origin`为远程仓库的简写。

> `git fetch`只会将数据下载到本地仓库,不会自动合并或修改当前工作区.

查看某个远程仓库:
```shell
$ git remote show <remote>
```
它会列出远程仓库URL与跟踪分支的信息. 在特定分支执行`git pull`会推送到哪个分支,列出了哪些远程分支不在你的本地,哪些远程分支已经在服务器中移除,以及执行`git pull`时哪些本地分支可以与它追踪的远程分支自动合并

远程仓库的移除与重命名:
```shell
$ git remote rename <old> <new>
$ git remote remove <remote>
```
一旦删除远程仓库,那么所有和这个远程仓库相关的远程跟踪分支以及配置信息都会一起被删除

## 标签

Git可以给历史中的某一个提交打上标签，一般用这个功能标记发布节点(v1.0, v2.0等)。

列出已有标签:
```shell
$ git tag
v1.0
v2.0
```

可以按照特定模式列出标签,如只列出以`v1`开头的标签:
```shell
$ git tag -l "v1.*"
v1.0
v1.1
```

Git支持两种标签,轻量(lightweight)标签和附注(annotated)标签

轻量标签像一个不会改变的分支,它只是某个特定提交的引用
附注标签是存储在Git数据库中的一个完整对象,它们是可以被校验的,其中包含打标签者的名字,电子邮件地址,日期时间,还有一个标签信息,并且可以使用GNU Privacy Guard(GPG)签名并验证

创建附注标签:
```shell
$ git tag -a v1.4 -m "my version 1.4"
```

`-m`选项指定了将会存储在标签中的信息

通过`git show`命令可以查看标签信息与之对应的提交信息
```shell
$ git show v1.4
```
输出显示了打标签者的信息、日期、附注信息和具体的提交信息

创建轻量标签:
```shell
$ git tag v1.4-lw
```
在轻量标签上运行`git show`,不会看到额外的信息,只会显示提交信息
> 本质上是将提交校验和存储到一个文件中——没有保存任何其他信息

默认情况下,`git push`并不会将标签传送到远程仓库服务器上,在创建完标签后需要显式地推送标签到共享服务器上:
```shell
$ git push origin v1.5
```

如果想要一次性推送多个标签,可以使用带有`--tags`选项的`git push`命令

删除本地标签:
```shell
$ git tag -d v1.4
```
如果也需要删除远程,执行:
```shell
$ git push origin --delete v1.4
```

如果想查看某个标签指向的文件版本,可以使用`git checkout`命令:
```shell
$ git checkout 2.0.0
```
这会使仓库处于“分离头指针”状态,在这种状态下,进行更改并提交,标签不会发生变化,当你的新提交将不属于任何分支,并且无法访问,除非通过确切的提交哈希值才能访问

Git中不存在随接触提交递增的`v123`之类的数字序列,如果为提交附加一个可读的名称,可以对其运行`git describe`命令,Git将会生成一个字符串,它由最近的标签名、自该标签之后的提交数目和提交部分SHA-1值构成:
```shell
$ git describe
v1.2-1-gf703
```
如果你在导出快照或者构建时,可以给出一个便于理解的命名
> 这个命令需要有注解的标签,要使用轻量标签,需要添加`--tags`选项

> 发布一个构建:
```shell
$ git archive master --prefix='project/' | gzip > `date +%Y%m%d`.tar.gz
```

## 分支原理

Git保存的不是文件的差异或变化,而是一系列不同时刻的**快照**

在进行提交操作时,git会保存一个提交对象——该对象会包含一个指向暂存内容快照的指针.

首次提交产生的提交对象没有父对象,普通提交操作产生的提交对象有一个父对象,而由多个分支合并产生的提交对象有多个父对象.

暂存操作回味每一个文件计算校验和,然后将当前版本的文件快照保存到Git仓库中(Git使用Blob对象存储),最终将校验和加入到暂存区域等待提交

当使用`git commit`时,Git会先计算每一个子目录的校验和,然后在Git仓库中将这些校验和保存为树对象,Git会创建一个提交对象,包含指向树对象的指针.

总结下,Git仓库中有5个对象:3个Blob对象(保存文件快照)、一个树对象(记录目录结构和blob对象索引)以及一个提交对象(包含指向前述树对象的指针和所有提交信息)


Git的分支,本质上仅仅是指向提交对象的可变指针.

> 缺个图

## 创建分支

Git创建分支,就是创建了一个可以移动的新指针:
```shell
$ git branch feat1
```

这会在当前提交对象上创建一个指针:
> 缺个图

Git通过一个名为HEAD的特殊指针,确定当前所处的分支.可以将HEAD想象为当前分支的别名.
> 缺个图

使用如下命令可查看各个分支当前所指的对象
```shell
$ git log --oneline --decorate
```

演示: 切换testing分支,提交,切换回master分支,展示HEAD的变化

`git checkout`做了两件事:
1. 将HEAD指向切换的分支
2. 将工作目录恢复成当前分支指向的快照内容

演示:master进行一次修改并提交
这个项目的提交历史已经产生了分叉,上面两次提交发生在不同分支
> 来个图展示下

你可以使用`git log`查看分叉历史:
```shell
$ git log --oneline --decorate --graph --all
```

由于Git的分支实质上仅是包含所指对象校验和的文件,所以分支的创建和销毁都很高效.
创建一个分支就相当于往一个文件中写入41个字节(40个字符的SHA-1校验和和一个换行符)

## 分支切换

在切换分支之前,保持好一个干净的状态.可以使用暂存(stashing)或修补提交(commit amending)

切换分支时Git会自动添加、删除、修改文件以确保此时你的工作目录和这个分支最后一次提交时一样.

## 分支合并

当你合并代码时,可能会注意到`Fast-forward`快进这个词.由于你要合并的分支所指向的提交时你说在的提交的直接后续,因此Git会直接将指针向前移动.
换句话说:当你试图合并两个分支时,如果顺着一个分支走下去能够到达另一个分支,那么Git在合并两者时,只会简单的将指针向前推进(指针右移),因为这种情况下的合并操作没有需要解决的分歧——这就叫做快进(fast-forward)
> 缺个图

如果你的开发历史从一个更早的地方开始分叉(diverged),即master分支所在的提交并不是feat1分支所在提交的直接祖先,Git不得不做一些额外的工作.
这种情况下,Git会使用两个分支的末端所指的快照以及两个分支的公共祖先,做一个简单的三方合并.和之前将分支向前推进不同的是,Git将此次三方合并的结果做了新的快照并自动创建一个新的提交指向它.这被称为一次合并提交,它的特别之处在于它有不止一个父提交.
> 缺个图

如果在两个不同分支,对同一个文件的同一部分进行了不同的修改,Git就没法干净的合并它们.
在合并时就会产生合并冲突.需要你解决合并产生的冲突后才能提交合并的结果.
> 演示下冲突发生的场景


任何因包含合并冲突而有待解决的问价,都会以未合并状态标识出来.Git会在有冲突的文件中加入标准的冲突解决标记,这样你可以打开这些包含冲突的文件并手动解决冲突.
> vscode 提供了[三路合并编辑器功能](https://code.visualstudio.com/updates/v1_69#_3-way-merge-editor)

> 来个截图展示合并冲突标记

在你解决了所有文件冲突之后,对每个文件使用`git add`命令来将其标记为冲突已解决.一旦暂存这些原本有冲突的文件,Git就会将它们标记为冲突已解决.

可以使用图形化工具来解决冲突,如`git mergetool`命令,它会启动一个图形化的合并工具,并且自动将所有未解决的文件标记为已解决.

## 分支模型

### 长期分支

长期分支是指在整个项目开发周期中都存在的分支,它们一般用于发布稳定的版本,或者作为向主分支合并的目标分支.
- master分支保留完全稳定的代码,可能仅仅是已经发布或即将发布的代码
- develop或next分支,被用来做后续开发或测试稳定性,这些分支不必保持绝对稳定,但一旦达到稳定状态,就能被合并入master分支
- 主题分支,用于开发某个特定的功能或任务,一旦完成,就可以合并入develop分支

> 缺个图

通常把这种分支模型想象未流水线,经过测试考验的提交会被合入到更加稳定的流水线中,最终合入master分支

你可以用这种方法维护不同层次的稳定性.一些大型项目还有个proposed或pu:proposed updates(建议更新)分支,它可能包含一些不成熟的内容,而不能进入next或者master分支.

### 主题分支

主题分支是一种短期分支,它被用来实现单一特性或其相关工作.

> 来个图

### 远程分支

远程跟踪分支是远程分支状态的引用,它们是你无法移动的本地引用.一旦你进行了网络通信,Git就会为你移动他们以准确反映远程仓库的状态
> 可以把他们类比书签🔖, 可以提醒你该分支在远程仓库中的位置就是你最后一次连接到他们的位置

远程分支以`<remote>/<branch>`的形式命名,比如`kol/dev_s141_20231221`分支在`origin`远程仓库中,那么它的远程跟踪分支就是`origin/kol/dev_s141_20231221`

> 远程仓库名字`origin`与默认分支名字`master`一样,在Git中没有特殊的含义.`origin`是你`git clone`时默认的远程仓库名字, 可以执行`git clone -o <name>`来指定其他名字
> 缺个图展示 远程与本地分支的联系

如果要与给定的远程仓库同步数据,运行`git fetch <remote>`命令,这个命令查找`origin`是哪个服务器并从中抓取本地没有的数据,更新本地数据库,移动`origin/<branch name>`指针到更新之后的位置

> 来个图展示`fetch`之后的状态

## 推送

当想要分享一个分支时,需要将其推送到有写入权限的远程仓库中.本地分支并不会自动与远程仓库同步,你必须显式地推送你想要分享的分支.

运行`git push <remote> <branch>`,如:
```shell
$ git push origin issue233
```

这个命令简化了一些概念:
> Git自动将issue233名字展开为`refs/heads/issue233:refs/heads/issue233`,这意味着“推送本地的issue233分支来更新远程仓库的issue233分支”

上面的命令与下方的等效:
```shell
$ git push origin issue233:issue233
```

如果不想将本地分支推送到同名的远程分支,可以运行:
```shell
$ git push origin issue233:server_issue233
```

每次从远程仓库抓取,本地不会自动生成一份可编辑副本,只有一个不可修改的`origin/server_issue233`指针
可以通过执行`git checkout -b issue233 origin/server_issue233`来创建一个本地分支,它指向`origin/server_issue233`指针所在的位置

## 跟踪分支

从一个远程跟踪分支检出一个本地分支会自动创建所谓的“跟踪分支”(它跟踪的分支被称为上游分支).
> 跟踪分支是与远程分支有直接关系的本地分支

如果在一个跟踪分支执行`git pull`,Git会自动识别去哪个服务器抓取、合并到哪个分支

要创建一个跟踪分支,可以使用`git checkout -b <branch> <remote>/<branch>`,也可使用快捷方式:
```shell
$ git checkout --track origin/issue233
```

如果尝试检出的分支不存在且远程分支有与之同名的,则Git会为你创建一个跟踪分支

你可以在任意时间修改跟踪的上游分支:
```shell
$ git branch -u origin/server_issue233
```

> 设置好上游分支后,可以通过简写`@{upstream}`或`@{u}`来引用上游分支.
> 例如:master分支的上游分支是origin/master,可以使用`git merge @{u}`取代`git merge origin/master`

要查看设置的所有跟踪分支,可以使用`git branch`的`-vv`选项:
```shell
$ git branch -vv
* develop 0997a9f [origin/develop: behind 147] fix: 确认跳转发布页参数
  release 65304cb [origin/release] Merge branch 'feat233' into 'release'
```

可以看到正在跟踪的远程分支是否领先/落后

> 这些数字的值并非准确,取决于你上次抓取的数据`git fetch`.
> 这个命令并没有联网,只是告诉你关于本地缓存的服务器数据

## 拉取

使用`git fetch`从服务器抓取本地没有的内容时,它并不会修改工作目录中的内容
它只会获取数据然后让你自己合并.

可以使用`git pull`进行拉取操作,可以理解为`git fetch`+`git merge`
如果当前是一个跟踪分支,`git pull`会查找当前分支跟踪的服务器与分支,从服务器抓取数据然后尝试合并入远程分支.

可以使用`git push origin --delete <branch>`删除远程分支

## 变基

在Git中有两种整合来自不同分支的修改方法:merge与rebase

整合分支最容易的方式是使用`git merge`命令,它会把两个分支的最新快照以及二者最近的共同祖先进行三方合并,并创建一个新的快照作为结果提交.
> 缺个图

使用`rebase`,会提取分叉的分支引入的补丁和修改,然后在另一分支的最新快照基础上应用一次.
> 使用`rebase`命令将提交到某一分支的所有修改都移到另一分支中,就好像“重新播放”一样

它的原理是首先找到这两个分支的最近共同祖先,然后对比当前分支相对于该祖先的历史提交,提取相应的修改并存为临时文件,然后将当前分支指向目标基底,最后以此将之前另存为临时文件的修改依序应用.
> 缺个图

这两种整合的方法最终结果没任何区别,但是变基的历史提交更为整洁.在查看经过变基的历史记录时可以看到,尽管实际开发工作是并行的,但他们看上去就像是串行的一样,提交历史是一条直线没有分叉

> 无论通过变基还是三方合并,整合的最终结果所指向的快照始终是一样的,只不过提交历史不同罢了.
> 变基是将一系列提交按照原有次序依次应用到另一个分支上,而合并是把最终结果合到一起

### 变基的风险

如果提交存在于你的仓库之外,而别人可能基于这些提交进行开发,那么不要进行变基!

变基操作的实质是丢弃一些现有的提交,然后相应地新建一些内容一样但实际上不同的提交.

如果你已经推送到某个仓库,而其他人也已经从该仓库拉取提交并进行了后续工作,如果此时你使用变基重写了提交历史,然后再尝试推送,就会发生冲突,因为你已经丢弃了别人的提交.


> 演示变基操作产生的后果

此时的处境就非常尴尬.如果其他人执行`git pull`命令,将合并来自两条提交历史的内容,生成一个新的合并提交.
将这一堆推送到服务器,实际上就将那些已经变基被抛弃的提交又找了回来(被其他人通过变基丢弃的提交)

### 用变基解决变基

如果真的遇到了类似处境,Git提供了一些高级魔法可以帮到你

如果团队中某人强制推送并覆盖了一些你基于的提交,你需要做的就是检查你做了哪些修改,以及他们覆盖了哪些修改.

> Git除了对整个提交计算SHA-1校验和以外,也对本次提交所引入的修改计算了校验和,即“patch-id”

回忆下前面变基的步骤: 有人推送了经过变基的提交,并丢弃了你本地开发所基于的一些提交

如果我们不执行合并,而是执行`git rebase origin/feat1`,Git将会:
- 检查哪些提交在我们的分支上独有
- 检查哪些提交不是合并操作的结果
- 检查哪些提交在对方覆盖更新时并没有被纳入目标分支
- 把查到的这些提交应用到`origin/feat1`上

与`git pull`(将相同内容又合并一次,生成一个新的提交)不同的是,在一个被变基并强制推送的分支再次执行变基,效果如图所示:
> 缺个图

本例还有个简单的方法:`git pull --rebase`,和先`git fetch`再执行`git rebase origin/feat1`等价

变基最好仅用于: 分支不会与其他人协作开发
如果不这样做,请确保其他人的拉取操作使用`git pull rebase`!

### 变基 OR 合并

只对尚未推送或分享给别人的本地修改执行变基清理历史,从不对已推送至别处的提交执行变基操作

### 其他杂项

当进行大量合并或变基时,Git提供了`rerere`工具,它可以记录解决冲突的方法,并在将来自动解决类似的冲突

`rerere`是`reuse recorded resolution`的缩写,重用已记录的冲突解决方案.当启用时,Git会维护一些成功合并之前和之后的镜像,但Git发现之前修复过类似的冲突时,便会使用之前的修复方案
启用:
```shell
$ git config --global rerere.enabled true
```


## 服务器上的Git

远程仓库通常只是一个裸仓库(bare repository)——即一个没有当前工作目录的仓库

1. 该仓库仅仅作为合作媒介,不需要从磁盘检查快照
2. 存放的只有Git元数据

简单来说,裸仓库就是你工作目录内的`.git`子目录内容

git可以通过四种协议传输数据: 本地协议、HTTP协议、SSH协议、Git协议

### 本地协议

本地协议是Git最常用的协议,它通常运行在共享文件系统上.

例如克隆一个本地版本库,可以执行如下命令:
```shell
$ git clone /srv/git/project.git
```

- 如果指定路径,Git会尝试使用[硬链接](https://zh.wikipedia.org/zh-cn/%E7%A1%AC%E9%93%BE%E6%8E%A5)或直接复制所需要的文件.
- 如果URL开头明确指定`file://`,Git会触发平时用于网络传输的进程,传输效率会降低

缺点:
1. 共享文件系统难以配置
2. 远程访问不便(需要挂载个远程磁盘)
3. 磁盘损坏会导致仓库丢失

### HTTP协议

- 智能HTTP协议:运行方式类似SSH及Git协议,运行在标准的HTTP/S端口上并能使用各种HTTP验证机制,既支持匿名服务,也支持传输时授权/加密
- 哑HTTP协议: Web服务器仅把裸版本库当作普通文件对待,提供文件服务

Git协议与SSH协议在此就不赘述

### 搭建Git服务器

在开始架设Git服务器之前,需要把现有仓库导出为裸仓库:
```shell
$ git clone --bare /path/to/repo.git
```

效果等同于:
```shell
$ cp -Rf my_project/.git my_project.git
```

有了裸仓库副本,接下来就是把裸仓库放到服务器上并执行:
```shell
$ git init --bare --shared
```
上述命令会创建一个裸仓库,并将其权限设置为可读写,这样其他用户就可以推送到这个仓库了
> 该命令不会摧毁任何提交、引用等内容

### GitWeb

创建了一个裸仓库,可以使用`gitweb`工具来浏览仓库内容(一个基于网页的简易查看器)

## Git提交准则

不应该包含任何空白错误,在提交前可以运行`git diff --check`,它会找到可能的空白错误并列出来
> 演示下

尝试为每个提交成为逻辑上的独立变更集
> 不要一次编码解决多个问题,然后提交为一个巨大的提交
> 将一个问题解决为一个提交,这样可以更容易地跟踪问题
> 如果一些改动同时修改了同一个文件,尝试使用`git add --patch`部分暂存文件

创建优质的提交信息将使Git的使用和协作更加轻松,参考:[Conventional Commits](https://www.conventionalcommits.org/zh-hans/v1.0.0/)
> 一般情况下,信息应当少于50个字符(25个汉字),接着是个空白行,然后是更详细的描述

类似如下格式:
```
type(scope?): subject

body?

footer?
```

## 制作提交简报

使用`git shortlog`命令可以生成一个简单的提交报告,包含从上次发布之后项目新增内容的修改日志(changelog)类文档,它会对给定范围内所有提交内容进行总结:
```shell
$ git shortlog --no-merges master --not v1.0.0
```

## Git工具

### 选择修订版本

使用Git只需提供SHA-1的前几个字符就能获的对应提交(不少于4个且无歧义)

使用`git show`检查提交:
```shell
$ git show 3224 --oneline
```

使用`git log`时加上`--abbrev-commit`查看简短且唯一的值(默认7位)
```shell
$ git log --abbrev-commit --oneline
```

如果想知道某个分支指向哪个特定SHA-1,可以使用`rev-parse`探测工具:
```shell
$ git rev-parse master
```

### 引用日志

`git reflog`命令可以查看引用日志,它会列出仓库中的引用更新记录,包括HEAD指针的移动和分支的创建与删除
```shell
$ git reflog
178ce584 (HEAD -> kol/dev_s141_20231221) HEAD@{0}: reset: moving to HEAD
178ce584 (HEAD -> kol/dev_s141_20231221) HEAD@{1}: commit (merge): Merge remote-tracking branch 'origin/release' into kol/dev_s141_20231221
fb7c56de HEAD@{2}: pull: Fast-forward
079716c9 HEAD@{3}: checkout: moving from refactor/task-hall to kol/dev_s141_20231221
4e1ae95d (refactor/task-hall) HEAD@{4}: commit: refactor: 任务大厅页面重写
b22962e1 (release) HEAD@{5}: checkout: moving from release to refactor/task-hall
b22962e1 (release) HEAD@{6}: pull --tags origin release: Fast-forward
ed37ade8 HEAD@{7}: checkout: moving from develop to release
bbbce290 (develop) HEAD@{8}: pull: Fast-forward
a36f6c07 HEAD@{9}: checkout: moving from release to develop
ed37ade8 HEAD@{10}: pull --tags origin release: Fast-forward
ca3e506a HEAD@{11}: checkout: moving from develop to release
```

每当你的HEAD指向位置发生了变化,Git就会将这个信息存储到引用日志中
可以通过reflog数据获取之前的提交历史(使用`@{n}`引用reflog中输出的提交记录)

```shell
$ git show @{5}
```
> 也可以用这个语法查看某个分支在一定时间前的位置: `git show master@{yesterday}`

> 引用日志只存在于本地仓库中,它只是记录你在自己仓库里做过什么的日志!
> 新克隆的仓库引用日志是空的!

### 祖先引用

你可以使用`^`操作符来引用一个提交的祖先,例如:
```shell
$ git show HEAD^
```
表示: HEAD的父提交

也可以在`^`后面添加一个数字指明第几个父提交:
```shell
$ git show HEAD^2
```
> 这个语法只适和于合并提交,因为合并提交有多个父提交.
> 合并提交第一父提交是你合并时所在的分支,第二父提交是你合并进来的分支

可以使用`~`波浪号,`HEAD^`等价于`HEAD~1`,即HEAD的第一个父提交,不同之处在于,`~`指定一个数字,表示向上回溯多少代祖先提交.`HEAD~2`代表"第一父提交的第一父提交"

也可以组合使用这两个语法:
```shell
$ git show HEAD~2^2
```
表示: HEAD的祖父提交的第二个父提交(假设是个合并提交)

### 提交区间

`git log master..develop` 在develop分支中但不在master分支中的提交

使用`^`或`--not`进行排除,下方三个命令是等价的:
```shell
$ git log master..develop
$ git log ^master develop
$ git log develop --not master
```
也可以指定多个分支:
```shell
$ git log feat1 develop --not master
```
包含feat1分支或develop分支, 不包含master分支的提交

`git log master...develop` 两个分支中的所有提交,但不包含两个分支共同的提交.配合`--left-right`参数显示每个提交位于哪一侧:
```shell
$ git log --left-right master...develop --oneline
```

### 贮藏

贮藏(有计划的储藏)会处理工作目录的脏的状态——即跟踪文件的修改与暂存文件的改动,但不包括未跟踪的文件(如新建文件),然后将未完成的修改保存在一个栈上,而你可以随时重新应用这些改动.

- 将新的贮藏推送到栈上,运行`git stash`或`git stash push`
- 查看贮藏的列表,运行`git stash list`
- 将贮藏重新应用`git stash apply stash@{2}` (不指定, 默认最近的贮藏)
- 丢弃指定贮藏,运行`git stash drop stash@{2}`
- 丢弃最近贮藏,运行`git stash pop`
- 丢弃所有贮藏,运行`git stash clear`

有用的选项:
- `--keep-index` 贮藏后,会继续保留在暂存区
- `--include-untracked`(`-u`) 贮藏后,会将未跟踪的文件也贮藏起来
- `--all`(`-a`) 包含忽略的文件(ignore)
- `--patch`交互式贮藏

基于贮藏创建一个分支: `git stash branch <branchname>`

### 清理工作目录

`git clean`命令用于从工作目录中删除所有**没有跟踪的文件**
> 谨慎使用!


一般使用`git clean`命令去除冗余文件或清理工作目录.使用`git clean -f -d`移除所有未追踪文件和空目录
> 使用`-f`需要配置`clean.requireForce`为`false`

可以配合`--dry-run`(`-n`)选项来查看将要删除的文件,而不是真的删除它们(做一次演习)
> 默认情况下,`git clean`不会删除`.gitignore`中指定的文件,如果想删除,需要使用`-x`选项
> 如果在工作目录复制或克隆了其他Git仓库(只模块submodule), `git clean -fd`都会拒绝删除,可以使用`-f`

### 搜索

`git grep`命令可以很方便地从提交历史、工作目录、甚至索引中查找一个字符串或者正则表达式

默认`git grep`查找工作目录的文件
- 传递`--line-number`(`-n`)输出Git找到的匹配行行号
- 不想打印匹配项,可以使用`--count`(`-c`)让Git输出概述信息
- 传入`--show-function`(`-p`)显示每个匹配字符串所在的方法或函数

```shell
$ git grep -n postcss
.stylelintrc.js:12:             customSyntax: 'postcss-scss',
.stylelintrc.js:21:             customSyntax: 'postcss-less',
package.json:113:        "postcss": "8.4.24",
package.json:114:        "postcss-flexbugs-fixes": "5.0.2",
package.json:115:        "postcss-less": "6.0.0",
package.json:116:        "postcss-scss": "4.0.6",
postcss.config.js:4:          'postcss-flexbugs-fixes',
postcss.config.js:5:          ['postcss-easy-import', { prefix: '_' }], // keep this first
```

## 重写历史

Git可以在本地随便重写历史记录,然而一旦推送了工作,除非必要不要重写公共历史

### 修改最后一次提交

一般常见于:简单地修改提交信息,或通过添加、移除或修改文件来更改提交实际的内容

需要修改最近一次提价的提交信息,可执行:
```shell
$ git commit --amend
```

上面的命令会将最后一次提交信息载入到编辑器中供你修改,但保存并关闭编辑器后,编辑器会用更新后的提交信息替换掉最后一次提交信息

> 该命令既可以修改**提交信息**也可以修改**提交内容**(新的更改放入暂存区)
> 该命令会改变提交的SHA-1校验和,类似于一个小的变基(!!!如果推送了,就不要再使用该命令)
> 可以使用`--no-edit`选项跳过修改提交信息

### 修改多个提交信息

Git没有提供改变历史的工具,但是可以使用变基工具来变基一系列提交
通过交互式变基工具,可以在任何想要修改的提交后停止,然后修改信息、添加文件或做任何想做的事情

> 实际代码演示

修改最近3次提交:
```shell
$ git rebase -i HEAD~3
```
在`HEAD~3..HEAD`范围内的每一个修改了提交信息的提交及其所有后裔都会被重写(!!!不要包含任何已被推送的提交——会产生一次变更的两个版本!)

需要注意的是交互式变基与正常使用log命令显示的提交顺序相反
> 交互式变基会从命令行中指定的提交(HEAD~3)开始,从上到下依次重演每个提交引入的修改

可以通过修改提交前的指令,来告诉Git如何处理提交:
- `edit` 修改提交信息(`git commit --amend`)
- `reword` 仅修改提交信息
- `squash` 与前一个提交合并
- `fixup` 与前一个提交合并,但丢弃提交信息

> 提交信息可以重新排序或移除

> 可以拆分提交,将一个提交拆分为多个提交
> 1. 使用`edit`指令指定要拆分的提交
> 2. 当变基到该提交时,使用`git reset HEAD^`撤销此次提交并将修改的文件取消暂存
> 3. 使用`git add`将修改的文件重新暂存并执行`git commit`创建一个新的提交
> 4. 重复上述步骤,直到完成所有拆分
> 5. 使用`git rebase --continue`继续变基

### 批量改写历史提交

如果想要修改多个提交信息,可以使用`git filter-branch`命令,它会重写历史中的所有提交,并将其应用到新的提交中

> `git filter-branch`有[很多陷阱](https://git-scm.com/docs/git-filter-branch/zh_HANS-CN#_warning),推荐使用`git-filter-repo`工具

> 演示下

---

从每个提交中移除一个文件,例如有人上传了巨大的文件或偶然提交了一个密码文件,想要从历史中移除它,可以执行:
```shell
$ git filter-branch --tree-filter 'rm -f passwords.txt' HEAD
```
`tree-filter`选项在检出项目每一个提交后运行指定命令然后重新提交结果.上面命令会从每个快照中移除`passwords.txt`文件,无论是否存在

最后可以看到Git重写树与提交然后移动分支指针.
> 最后先在一个测试分支中运行,当验证无误后硬重置master分支
> 需要让`filter-branch`在所有分支上运行,可以传递`--all`选项

> 可以使用[BFG Repo-Cleaner](https://rtyley.github.io/bfg-repo-cleaner/)替代

---

将某个子目录独立出来作为新的Git代码库,可以使用`subdirectory-filter`选项:
```shell
$ git filter-branch --subdirectory-filter apps/miniapp HEAD
```
现在新项目根目录是`apps/miniapp`目录,Git会自动移除所有不影响子目录的提交

---

假设你所在组织准备开源内部项目(Gitlab->Github),打算保留所有做出贡献的人的提交,但是内部邮箱与Github邮箱不一致,可以使用`--commit-filter`选项修改提交信息:
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
这会遍历并重写每一个提交来包含新邮箱地址.因为邮箱包含了他们父提交的SHA-1校验和,这个命令会修改你的历史中的每一个提交的SHA-1校验和,而不仅仅只是匹配邮箱地址的提交

## 重置

先看看下Git的3棵树🌲:
| 树       | 用途                            |
| -------- | ------------------------------- |
| HEAD     | 上次提交的快照,下次提交的父节点 |
| 索引     | 预期的下次提交快照              |
| 工作目录 | 沙盒                            |

### HEAD

HEAD 可简单理解为: 该分支最后一次提交的快照

> `git cat-file -p ^HEAD` 查看HEAD指向的提交
> `git ls-tree -r HEAD` 查看HEAD指向的提交包含的文件

### 索引

索引是**预期的下次提交**,即Git的“暂存区”,运行`git commit`时Git看上去的样子

Git将上次检出到工作目录中的所有文件填充到索引区,接着执行`git commit`将它们转换为树来用作新的提交

> `git ls-files -s`查看索引中的文件

### 工作目录

通常也被称为**工作区**

前面两颗树以一种高效但并不直观的方式存储在`.git`目录中,工作目录会将它们接包为实际的文件以便编辑.

可以把工作目录当作沙盒,再将修改提交到暂存区并记录到历史之前,可以随意更改

### 工作流程

经典的Git工作流程就是通过操纵这3个区域以更加连续的状态记录项目快照的:
> 缺个图
> 做个小动画描述下

切换分支或克隆时, 会修改HEAD指向新的分支引用,将索引填充为该次提交新的快照,然后将索引的内容复制到工作目录中

### reset的作用

### soft

(ps: checkout改变HEAD自身),本质是撤销了上一次git commit 命令.
以`git reset --soft HEAD~`为例(`--soft`选项不碰索引文件或工作树,只将头部重置,这养所有更改都是“已修改待提交状态”)

当你运行`git commit`时,Git会创建一个新的提交,并移动HEAD指向这个新提交,当reset回`HEAD~`(HEAD的父节点)时,其实就是把该分支移动回原来的位置,而不会改变索引和工作目录

### mixed

`git reset --mixed HEAD~`(`--mixed`选项会重置索引,但不会重置工作目录,即保留已修改文件,但不标记为提交):

撤销上一提交、取消暂存所有东西(回滚到了`git add`、`git commit`命令执行之前)

### hard

`git reset --hard HEAD~`(`--hard`选项会重置索引和工作目录,任何未跟踪文件、已修改文件都会被丢弃):

其他任何形式的`reset`调用都会轻松撤销,但是`--hard`选项不能,它会强制覆盖工作目录内的文件
> 如果Git数据库中的提交内留有对应文件的快照,可以通过`reflog`找回,若该文件未提交,Git会覆盖它从而无法恢复

总结下reset的三种模式:
- `--soft` 仅移动HEAD指针指向
- `--mixed` 移动HEAD指针和重置索引(取消暂存文件,与`git add`相反)
- `--hard` 移动HEAD指针、重置索引和重置工作目录

### 使用`reset`压缩提交

如果提交了3次,但是想要将它们合并为一个提交,可以使用`reset`命令:
```shell
$ git reset --soft HEAD~3
```

### `checkout`与`reset`的区别

和`reset`一样,`checkout`也可以操纵三棵树,但是它的行为不同

1. `git checkout <branch>`与`git reset --hard <branch>`类似,但是`checkout`对工作目录是安全的,它会通过检查来确保不会将以更改文件丢掉(先试着简单合),而`reset`则会强制覆盖工作目录
2. `git reset master`, develop自身和master指向同一个提交;`git checkout master`, develop不会移动,HEAD自身会移动(指向master)
> 缺个图

## 撤销合并

如果这个不想要的合并提交只存在于你的本地仓库,最简单的方案就是移动分支到你想要指向的地方:
```shell
$ git reset --hard HEAD~
```
这会经历3步:
1. 移动HEAD指向的分支
2. 使索引与HEAD指向的分支一致
3. 使工作目录与索引一致
缺点是其他人已经有你将要重写的历史,会丢失提交

也可以使用还原提交(revert):
```shell
$ git revert -m 1 HEAD
```
`-m`(mainline)选项指定要保留的父节点.当引入一个合并到HEAD,这次合并提交会引入两个父节点:HEAD和另一个分支,`-m 1`选项指定HEAD为主分支,另一个分支为次分支

> 画个图

新的提交和主分支有完全一样的内容,所有从这开始就想合并从未发生过(除了`现在还没合并`的提交依旧在HEAD历史中),如果再次尝试合并,会提示已经是最新的了

这产生了新的问题,如果在次分支增加新的提交并再次合并,Git只会引入被还原合并之后的修改
解决办法是撤销之前撤销产生的提交:
```shell
$ git revert revert_commit
```


## FAQ

### HEAD、目录树、索引、工作目录之间的关系

- HEAD指向当前分支的最后一次提交
- 目录树是一个包含文件名和文件快照的列表
- 索引是暂存区域,保存了下次将提交的文件列表信息
- 工作目录是对项目的某个版本独立提取出来的内容
- 三者之间的关系:
  - HEAD指向当前分支,当前分支指向目录树,目录树指向索引,索引指向工作目录
  - 当执行`git add`时,索引会更新为最新状态
  - 当执行`git commit`时,目录树会更新为最新状态
  - 当执行`git checkout`时,工作目录会更新为最新状态

