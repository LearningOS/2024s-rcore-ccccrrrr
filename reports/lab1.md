# ch3实验

## 思路和实现

我实现了一个获取当前正在运行应用系统调用信息和运行时长的功能。为了实现这一功能，首先在`TaskControlBlock`这一应用基本信息块中添加系统调用数组，和创建时间的条目。当一个应用第一次变为RUNNING状态时，需要记录它的创建时间。当系统调用函数运行时，需要根据系统调用code更新数组。

## 简答作业

1. 正确进入 U 态后，程序的特征还应有：使用 S 态特权指令，访问 S 态寄存器后会报错。 请同学们可以自行测试这些内容（运行 [三个 bad 测例 (ch2b_bad_*.rs)](https://github.com/LearningOS/rCore-Tutorial-Test-2024S/tree/master/src/bin) ）， 描述程序出错行为，同时注意注明你使用的 sbi 及其版本。
   答：
   这三个应用已经在link_app.S中加入了，不需要再做修改。

   + Ch2b_bad_address报错：PageFault in application, bad addr = 0x0, bad instruction = 0x804003ac, kernel killed it.程序直接在0x0地址上写0。
   + Ch2b_bad_instructions: IllegalInstruction in application。程序在U模式下使用没有权限的sret指令。
   + Ch2b_bad_register:IllegalInstruction in application, kernel killed it.程序在U模式下肚去S模式的寄存器sstatus。

2. 深入理解 [trap.S](https://github.com/LearningOS/rCore-Tutorial-Code-2024S/blob/ch3/os/src/trap/trap.S) 中两个函数 `__alltraps` 和 `__restore` 的作用，并回答如下问题:

   1. L40：刚进入 `__restore` 时，`a0` 代表了什么值。请指出 `__restore` 的两种使用情景。
      答：a0调用函数的第一个参数，即`current_task_cx_ptr`，是之前正在执行的应用，即将被暂停执行。
      1.调用`__alltraps`函数，在执行`trap_handler`后会执行`restore`；2.执行`__switch`，也就是应用之间切换时需要执行`restore`。

   2. L43-L48：这几行汇编代码特殊处理了哪些寄存器？这些寄存器的的值对于进入用户态有何意义？请分别解释。

      ```
      ld t0, 32*8(sp)
      ld t1, 33*8(sp)
      ld t2, 2*8(sp)
      csrw sstatus, t0
      csrw sepc, t1
      csrw sscratch, t2
      ```

   sstatus的SPP字段给出Trap发生之前CPU处在哪个特权级信息。模式切换时，会根据它的特权级信息检查。

   sepc：当Trap是一个异常时，记录Trap发生之前执行的最后一条指令的地址。在trap执行好后，会从这一条指令继续执行。

   sscratch在U模式指向内核栈，S模式指向用户栈。会在S和U模式切换时和sp的值交换，保证模式切换的正确性。

   3. L50-L56：为何跳过了 `x2` 和 `x4`？

   ```
   ld x1, 1*8(sp)
   ld x3, 3*8(sp)
   .set n, 5
   .rept 27
      LOAD_GP %n
      .set n, n+1
   .endr
   ```

   `x2`是`sp`，会在后面的`csrrw sp, sscratch, sp`恢复。`x4`是Thread Pointer，本次代码修改不涉及线程，不需要考虑该寄存器。

   4. L60：该指令之后，`sp` 和 `sscratch` 中的值分别有什么意义？

   ```
   csrrw sp, sscratch, sp
   ```

   csrrw指令将sp和sscratch寄存器的值交换。

   5. `__restore`：中发生状态切换在哪一条指令？为何该指令执行之后会进入用户态？

   发生状态切换是sret。在sret之前，它已经完成了准备运行应用的寄存器恢复，将用户栈正确确定，准备好重新开始运行机器的地址，所以能进入用户态。

   6. L13：该指令之后，`sp` 和 `sscratch` 中的值分别有什么意义？

   ```
   csrrw sp, sscratch, sp
   ```

   csrrw指令将sp和sscratch寄存器的值交换。

   7. 从 U 态进入 S 态是哪一条指令发生的？
      在`ecall $(trap code)`指令发生的。用户应用使用系统调用时，会使用ecall指令。在本实验中，内核为了避免一个应用占据太长时间，在应用运行一段时间后会换一个应用执行，内核会主动从U态变为S态。

## 声明

1. 在完成本次实验的过程（含此前学习的过程）中，我曾分别与以下各位就（与本次实验相关的）以下方面做过交流，还在代码中对应的位置以注释形式记录了具体的交流对象及内容：无

2. 此外，我也参考了以下资料，还在代码中对应的位置以注释形式记录了具体的参考来源及内容：无

3. 我独立完成了本次实验除以上方面之外的所有工作，包括代码与文档。我清楚地知道，从以上方面获得的信息在一定程度上降低了实验难度，可能会影响起评分。

4. 我从未使用过他人的代码，不管是原封不动地复制，还是经过了某些等价转换。我未曾也不会向他人（含此后各届同学）复制或公开我的实验代码，我有义务妥善保管好它们。我提交至本实验的评测系统的代码，均无意于破坏或妨碍任何计算机系统的正常运转。我清楚地知道，以上情况均为本课程纪律所禁止，若违反，对应的实验成绩将按“-100”分计。