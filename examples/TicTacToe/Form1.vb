Public Class Form1
    Dim turn As String
    Dim gameOver As Boolean

    Private Sub Form1_Load() Handles Me.Load
        ResetGame()
    End Sub

    Private Sub ResetGame()
        turn = "X"
        gameOver = False
        lblStatus.Caption = "Your Turn (X)"
        
        btn0.Caption = ""
        btn1.Caption = ""
        btn2.Caption = ""
        btn3.Caption = ""
        btn4.Caption = ""
        btn5.Caption = ""
        btn6.Caption = ""
        btn7.Caption = ""
        btn8.Caption = ""
        
        btn0.Enabled = True
        btn1.Enabled = True
        btn2.Enabled = True
        btn3.Enabled = True
        btn4.Enabled = True
        btn5.Enabled = True
        btn6.Enabled = True
        btn7.Enabled = True
        btn8.Enabled = True
    End Sub

    Private Sub btn0_Click() Handles btn0.Click
        HandleClick(btn0)
    End Sub

    Private Sub btn1_Click() Handles btn1.Click
        HandleClick(btn1)
    End Sub

    Private Sub btn2_Click() Handles btn2.Click
        HandleClick(btn2)
    End Sub

    Private Sub btn3_Click() Handles btn3.Click
        HandleClick(btn3)
    End Sub

    Private Sub btn4_Click() Handles btn4.Click
        HandleClick(btn4)
    End Sub

    Private Sub btn5_Click() Handles btn5.Click
        HandleClick(btn5)
    End Sub

    Private Sub btn6_Click() Handles btn6.Click
        HandleClick(btn6)
    End Sub

    Private Sub btn7_Click() Handles btn7.Click
        HandleClick(btn7)
    End Sub

    Private Sub btn8_Click() Handles btn8.Click
        HandleClick(btn8)
    End Sub

    Private Sub HandleClick(btn As Object)
        If gameOver Then
            Exit Sub
        End If
        If turn <> "X" Then
            Exit Sub
        End If
        If btn.Caption <> "" Then
            Exit Sub
        End If
        
        btn.Caption = "X"
        CheckWin()
        
        If Not gameOver Then
            turn = "O"
            lblStatus.Caption = "Computer thinking..."
            ComputerMove()
        End If
    End Sub

    Private Sub ComputerMove()
        If gameOver Then
            Exit Sub
        End If

        ' Try to win
        If TryMove("O") Then
            CheckWin()
            If Not gameOver Then
                turn = "X"
                lblStatus.Caption = "Your Turn (X)"
            End If
            Exit Sub
        End If

        ' Try to block player
        If TryMove("X") Then
            CheckWin()
            If Not gameOver Then
                turn = "X"
                lblStatus.Caption = "Your Turn (X)"
            End If
            Exit Sub
        End If

        ' Take center if free
        If btn4.Caption = "" Then
            btn4.Caption = "O"
            CheckWin()
            If Not gameOver Then
                turn = "X"
                lblStatus.Caption = "Your Turn (X)"
            End If
            Exit Sub
        End If

        ' Take a corner
        If btn0.Caption = "" Then
            btn0.Caption = "O"
            CheckWin()
            If Not gameOver Then
                turn = "X"
                lblStatus.Caption = "Your Turn (X)"
            End If
            Exit Sub
        End If
        If btn2.Caption = "" Then
            btn2.Caption = "O"
            CheckWin()
            If Not gameOver Then
                turn = "X"
                lblStatus.Caption = "Your Turn (X)"
            End If
            Exit Sub
        End If
        If btn6.Caption = "" Then
            btn6.Caption = "O"
            CheckWin()
            If Not gameOver Then
                turn = "X"
                lblStatus.Caption = "Your Turn (X)"
            End If
            Exit Sub
        End If
        If btn8.Caption = "" Then
            btn8.Caption = "O"
            CheckWin()
            If Not gameOver Then
                turn = "X"
                lblStatus.Caption = "Your Turn (X)"
            End If
            Exit Sub
        End If

        ' Take any open spot
        If btn1.Caption = "" Then
            btn1.Caption = "O"
        ElseIf btn3.Caption = "" Then
            btn3.Caption = "O"
        ElseIf btn5.Caption = "" Then
            btn5.Caption = "O"
        ElseIf btn7.Caption = "" Then
            btn7.Caption = "O"
        End If

        CheckWin()
        If Not gameOver Then
            turn = "X"
            lblStatus.Caption = "Your Turn (X)"
        End If
    End Sub

    ' Try to find a winning move for the given mark, and place "O" there
    Private Function TryMove(mark As String) As Boolean
        ' Check all 8 lines for two-in-a-row with one empty
        If TryLine(btn0, btn1, btn2, mark) Then Return True
        If TryLine(btn3, btn4, btn5, mark) Then Return True
        If TryLine(btn6, btn7, btn8, mark) Then Return True
        If TryLine(btn0, btn3, btn6, mark) Then Return True
        If TryLine(btn1, btn4, btn7, mark) Then Return True
        If TryLine(btn2, btn5, btn8, mark) Then Return True
        If TryLine(btn0, btn4, btn8, mark) Then Return True
        If TryLine(btn2, btn4, btn6, mark) Then Return True
        Return False
    End Function

    ' If two cells have 'mark' and one is empty, place "O" in the empty one
    Private Function TryLine(b1 As Object, b2 As Object, b3 As Object, mark As String) As Boolean
        If b1.Caption = mark And b2.Caption = mark And b3.Caption = "" Then
            b3.Caption = "O"
            Return True
        End If
        If b1.Caption = mark And b3.Caption = mark And b2.Caption = "" Then
            b2.Caption = "O"
            Return True
        End If
        If b2.Caption = mark And b3.Caption = mark And b1.Caption = "" Then
            b1.Caption = "O"
            Return True
        End If
        Return False
    End Function

    Private Sub CheckWin()
        If CheckLine(btn0, btn1, btn2) Then
            Exit Sub
        End If
        If CheckLine(btn3, btn4, btn5) Then
            Exit Sub
        End If
        If CheckLine(btn6, btn7, btn8) Then
            Exit Sub
        End If
        
        If CheckLine(btn0, btn3, btn6) Then
            Exit Sub
        End If
        If CheckLine(btn1, btn4, btn7) Then
            Exit Sub
        End If
        If CheckLine(btn2, btn5, btn8) Then
            Exit Sub
        End If
        
        If CheckLine(btn0, btn4, btn8) Then
            Exit Sub
        End If
        If CheckLine(btn2, btn4, btn6) Then
            Exit Sub
        End If
        
        If btn0.Caption <> "" And btn1.Caption <> "" And btn2.Caption <> "" And btn3.Caption <> "" And btn4.Caption <> "" And btn5.Caption <> "" And btn6.Caption <> "" And btn7.Caption <> "" And btn8.Caption <> "" Then
             lblStatus.Caption = "It's a Draw!"
             gameOver = True
        End If
    End Sub

    Private Function CheckLine(b1 As Object, b2 As Object, b3 As Object) As Boolean
        If b1.Caption <> "" And b1.Caption = b2.Caption And b2.Caption = b3.Caption Then
            If b1.Caption = "X" Then
                lblStatus.Caption = "You Win!"
            Else
                lblStatus.Caption = "Computer Wins!"
            End If
            gameOver = True
            DisableAll()
            Return True
        End If
        Return False
    End Function

    Private Sub DisableAll()
        btn0.Enabled = False
        btn1.Enabled = False
        btn2.Enabled = False
        btn3.Enabled = False
        btn4.Enabled = False
        btn5.Enabled = False
        btn6.Enabled = False
        btn7.Enabled = False
        btn8.Enabled = False
    End Sub

    Private Sub btnReset_Click() Handles btnReset.Click
        ResetGame()
    End Sub
End Class
