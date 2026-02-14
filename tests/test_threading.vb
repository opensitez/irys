' Test: Threading, Tasks, and BackgroundWorker
Imports System.Threading
Imports System.ComponentModel

Module TestThreading
    Dim passCount As Integer = 0
    Dim failCount As Integer = 0
    Dim bgwDoWorkCalled As Boolean = False
    Dim bgwCompletedCalled As Boolean = False
    Dim bgwProgressValue As Integer = 0
    Dim bgwResult As String = ""

    Sub AssertEqual(actual As Object, expected As Object, testName As String)
        If actual.ToString() = expected.ToString() Then
            Console.WriteLine("  PASS: " & testName)
            passCount = passCount + 1
        Else
            Console.WriteLine("  FAIL: " & testName & " (expected=" & expected.ToString() & " actual=" & actual.ToString() & ")")
            failCount = failCount + 1
        End If
    End Sub

    Sub Main()
        ' ===== Thread.Sleep =====
        Console.WriteLine("Test: Thread.Sleep")
        Dim sw As New System.Diagnostics.Stopwatch()
        sw.Start()
        Thread.Sleep(50)
        sw.Stop()
        AssertEqual(sw.ElapsedMilliseconds >= 40, True, "Thread.Sleep: paused ~50ms")

        ' ===== Task.Run =====
        Console.WriteLine("Test: Task.Run")
        Dim t = Task.Run(Function() 42)
        AssertEqual(t.IsCompleted, True, "Task.Run: IsCompleted")
        AssertEqual(t.Result, 42, "Task.Run: Result = 42")
        AssertEqual(t.Status, "RanToCompletion", "Task.Run: Status")

        ' ===== Task.Delay =====
        Console.WriteLine("Test: Task.Delay")
        Dim td = Task.Delay(10)
        AssertEqual(td.IsCompleted, True, "Task.Delay: IsCompleted")

        ' ===== Task.FromResult =====
        Console.WriteLine("Test: Task.FromResult")
        Dim tf = Task.FromResult("hello")
        AssertEqual(tf.Result, "hello", "Task.FromResult: Result")
        AssertEqual(tf.IsCompleted, True, "Task.FromResult: IsCompleted")

        ' ===== Task.WhenAll =====
        Console.WriteLine("Test: Task.WhenAll")
        Dim t1 = Task.FromResult(1)
        Dim t2 = Task.FromResult(2)
        Dim tAll = Task.WhenAll(t1, t2)
        AssertEqual(tAll.IsCompleted, True, "Task.WhenAll: IsCompleted")

        ' ===== Interlocked.Increment =====
        Console.WriteLine("Test: Interlocked")
        Dim counter As Integer = 10
        Dim newVal As Integer = Interlocked.Increment(counter)
        AssertEqual(newVal, 11, "Interlocked.Increment: returns 11")
        AssertEqual(counter, 11, "Interlocked.Increment: variable updated")

        ' ===== Interlocked.Decrement =====
        newVal = Interlocked.Decrement(counter)
        AssertEqual(newVal, 10, "Interlocked.Decrement: returns 10")
        AssertEqual(counter, 10, "Interlocked.Decrement: variable updated")

        ' ===== Interlocked.Exchange =====
        Dim oldVal As Integer = Interlocked.Exchange(counter, 99)
        AssertEqual(oldVal, 10, "Interlocked.Exchange: returns old value 10")
        AssertEqual(counter, 99, "Interlocked.Exchange: variable set to 99")

        ' ===== Interlocked.CompareExchange =====
        oldVal = Interlocked.CompareExchange(counter, 200, 99)
        AssertEqual(oldVal, 99, "Interlocked.CompareExchange: returns old value")
        AssertEqual(counter, 200, "Interlocked.CompareExchange: swapped to 200")

        ' CompareExchange with mismatch — should NOT swap
        oldVal = Interlocked.CompareExchange(counter, 300, 99)
        AssertEqual(counter, 200, "Interlocked.CompareExchange: no swap on mismatch")

        ' ===== BackgroundWorker =====
        Console.WriteLine("Test: BackgroundWorker")
        Dim bgw As New BackgroundWorker()
        AssertEqual(bgw.IsBusy, False, "BGW: not busy initially")
        bgw.WorkerReportsProgress = True
        bgw.WorkerSupportsCancellation = True

        AddHandler bgw.DoWork, AddressOf BGW_DoWork
        AddHandler bgw.RunWorkerCompleted, AddressOf BGW_Completed
        AddHandler bgw.ProgressChanged, AddressOf BGW_Progress

        bgw.RunWorkerAsync("test-input")

        AssertEqual(bgwDoWorkCalled, True, "BGW: DoWork handler called")
        AssertEqual(bgwCompletedCalled, True, "BGW: Completed handler called")
        AssertEqual(bgwResult, "done: test-input", "BGW: Result passed through")
        AssertEqual(bgwProgressValue, 50, "BGW: ProgressChanged fired")
        AssertEqual(bgw.IsBusy, False, "BGW: not busy after completion")

        ' ===== ThreadPool =====
        Console.WriteLine("Test: ThreadPool")
        ' QueueUserWorkItem runs synchronously in our interpreter
        ThreadPool.QueueUserWorkItem(Sub(state) Console.Write(""))
        passCount = passCount + 1
        Console.WriteLine("  PASS: ThreadPool.QueueUserWorkItem: executed without error")

        ' ===== Summary =====
        Console.WriteLine("")
        Console.WriteLine("Results: " & passCount & " passed, " & failCount & " failed")
        If failCount = 0 Then
            Console.WriteLine("SUCCESS: All threading tests passed!")
        Else
            Console.WriteLine("FAILURE: Some tests failed")
        End If
    End Sub

    Sub BGW_DoWork(sender As Object, e As DoWorkEventArgs)
        bgwDoWorkCalled = True
        ' Access the argument
        Dim input As String = e.Argument.ToString()
        ' Report progress
        sender.ReportProgress(50)
        ' Set the result
        e.Result = "done: " & input
    End Sub

    Sub BGW_Completed(sender As Object, e As RunWorkerCompletedEventArgs)
        bgwCompletedCalled = True
        bgwResult = e.Result.ToString()
    End Sub

    Sub BGW_Progress(sender As Object, e As ProgressChangedEventArgs)
        bgwProgressValue = e.ProgressPercentage
    End Sub
End Module
