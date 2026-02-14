' Test: Misc Gaps — Environment, Activator, CallByName, Guid, Stopwatch
Imports System

Module TestMiscGaps
    Dim passCount As Integer = 0
    Dim failCount As Integer = 0

    Sub AssertEqual(actual As Object, expected As Object, testName As String)
        If actual.ToString() = expected.ToString() Then
            Console.WriteLine("  PASS: " & testName)
            passCount = passCount + 1
        Else
            Console.WriteLine("  FAIL: " & testName & " (expected=" & expected.ToString() & " actual=" & actual.ToString() & ")")
            failCount = failCount + 1
        End If
    End Sub

    Sub AssertTrue(condition As Boolean, testName As String)
        If condition Then
            Console.WriteLine("  PASS: " & testName)
            passCount = passCount + 1
        Else
            Console.WriteLine("  FAIL: " & testName)
            failCount = failCount + 1
        End If
    End Sub

    Sub Main()
        ' ===== Environment =====
        Console.WriteLine("Test: Environment")
        AssertTrue(Environment.UserName <> "", "Environment.UserName not empty")
        AssertTrue(Environment.MachineName <> "", "Environment.MachineName not empty")
        AssertTrue(Environment.OSVersion <> "", "Environment.OSVersion not empty")
        AssertTrue(Environment.ProcessorCount > 0, "Environment.ProcessorCount > 0")
        AssertTrue(Environment.NewLine <> "", "Environment.NewLine not empty")
        AssertTrue(Environment.TickCount > 0, "Environment.TickCount > 0")
        AssertTrue(Environment.Is64BitOperatingSystem = True, "Environment.Is64BitOS")
        AssertTrue(Environment.CurrentDirectory <> "", "Environment.CurrentDirectory not empty")

        ' Environment.GetEnvironmentVariable
        Environment.SetEnvironmentVariable("VYBE_TEST_VAR", "hello123")
        AssertEqual(Environment.GetEnvironmentVariable("VYBE_TEST_VAR"), "hello123", "Environment.GetEnvironmentVariable")

        ' ===== Guid =====
        Console.WriteLine("Test: Guid")
        Dim g1 = Guid.NewGuid()
        Dim g2 = Guid.NewGuid()
        AssertTrue(g1.ToString().Length > 30, "Guid.NewGuid: valid format")
        ' Guids should be different (in practice)
        ' (We won't test uniqueness strictly since timing-based)

        ' ===== Stopwatch =====
        Console.WriteLine("Test: Stopwatch")
        Dim sw As New System.Diagnostics.Stopwatch()
        sw.Start()
        Thread.Sleep(20)
        sw.Stop()
        AssertTrue(sw.ElapsedMilliseconds >= 10, "Stopwatch: elapsed >= 10ms")
        AssertEqual(sw.IsRunning, False, "Stopwatch: stopped")

        sw.Reset()
        AssertEqual(sw.ElapsedMilliseconds, 0, "Stopwatch: reset to 0")

        Dim sw2 = System.Diagnostics.Stopwatch.StartNew()
        Thread.Sleep(10)
        sw2.Stop()
        AssertTrue(sw2.ElapsedMilliseconds >= 5, "Stopwatch.StartNew: elapsed > 0")

        ' ===== Activator.CreateInstance =====
        Console.WriteLine("Test: Activator.CreateInstance")
        Dim al = Activator.CreateInstance("ArrayList")
        al.Add("x")
        al.Add("y")
        AssertEqual(al.Count, 2, "Activator: ArrayList has 2 items")

        Dim dict = Activator.CreateInstance("Dictionary")
        dict.Add("key1", "val1")
        AssertEqual(dict("key1"), "val1", "Activator: Dictionary works")

        ' Create a custom class via Activator
        Dim obj = Activator.CreateInstance("TestWidget")
        AssertTrue(obj IsNot Nothing, "Activator: generic object created")

        ' ===== CallByName =====
        Console.WriteLine("Test: CallByName")
        Dim person As New TestPerson()
        person.Name = "Alice"
        person.Age = 30

        ' CallByName — Get property (callType = 2)
        Dim nameVal = CallByName(person, "Name", 2)
        AssertEqual(nameVal, "Alice", "CallByName Get: Name = Alice")

        ' CallByName — Set property (callType = 4)
        CallByName(person, "Name", 4, "Bob")
        AssertEqual(person.Name, "Bob", "CallByName Set: Name = Bob")

        ' ===== Process.Start (basic) =====
        Console.WriteLine("Test: Process")
        Dim p = System.Diagnostics.Process.Start("echo", "hello")
        AssertTrue(p.Id > 0, "Process.Start: has Id")

        ' Process instance methods
        p.WaitForExit()
        p.Close()

        ' ===== Summary =====
        Console.WriteLine("")
        Console.WriteLine("Results: " & passCount & " passed, " & failCount & " failed")
        If failCount = 0 Then
            Console.WriteLine("SUCCESS: All misc gaps tests passed!")
        Else
            Console.WriteLine("FAILURE: Some tests failed")
        End If
    End Sub
End Module

Class TestPerson
    Public Name As String
    Public Age As Integer
End Class
