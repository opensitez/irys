Partial Class Form1
    Inherits System.Windows.Forms.Form

    Friend WithEvents btn0 As System.Windows.Forms.Button
    Friend WithEvents btn1 As System.Windows.Forms.Button
    Friend WithEvents btn2 As System.Windows.Forms.Button
    Friend WithEvents btn3 As System.Windows.Forms.Button
    Friend WithEvents btn4 As System.Windows.Forms.Button
    Friend WithEvents btn5 As System.Windows.Forms.Button
    Friend WithEvents btn6 As System.Windows.Forms.Button
    Friend WithEvents btn7 As System.Windows.Forms.Button
    Friend WithEvents btn8 As System.Windows.Forms.Button
    Friend WithEvents lblStatus As System.Windows.Forms.Label
    Friend WithEvents btnReset As System.Windows.Forms.Button

    Private Sub InitializeComponent()
        Me.btn0 = New System.Windows.Forms.Button()
        Me.btn1 = New System.Windows.Forms.Button()
        Me.btn2 = New System.Windows.Forms.Button()
        Me.btn3 = New System.Windows.Forms.Button()
        Me.btn4 = New System.Windows.Forms.Button()
        Me.btn5 = New System.Windows.Forms.Button()
        Me.btn6 = New System.Windows.Forms.Button()
        Me.btn7 = New System.Windows.Forms.Button()
        Me.btn8 = New System.Windows.Forms.Button()
        Me.lblStatus = New System.Windows.Forms.Label()
        Me.btnReset = New System.Windows.Forms.Button()
        Me.SuspendLayout()
        Me.btn0.Location = New System.Drawing.Point(20, 50)
        Me.btn0.Size = New System.Drawing.Size(60, 60)
        Me.btn0.Text = ""
        Me.btn0.Name = "btn0"
        Me.btn0.TabIndex = 0
        Me.Controls.Add(Me.btn0)
        Me.btn1.Location = New System.Drawing.Point(80, 50)
        Me.btn1.Size = New System.Drawing.Size(60, 60)
        Me.btn1.Text = ""
        Me.btn1.Name = "btn1"
        Me.btn1.TabIndex = 1
        Me.Controls.Add(Me.btn1)
        Me.btn2.Location = New System.Drawing.Point(140, 50)
        Me.btn2.Size = New System.Drawing.Size(60, 60)
        Me.btn2.Text = ""
        Me.btn2.Name = "btn2"
        Me.btn2.TabIndex = 2
        Me.Controls.Add(Me.btn2)
        Me.btn3.Location = New System.Drawing.Point(20, 110)
        Me.btn3.Size = New System.Drawing.Size(60, 60)
        Me.btn3.Text = ""
        Me.btn3.Name = "btn3"
        Me.btn3.TabIndex = 3
        Me.Controls.Add(Me.btn3)
        Me.btn4.Location = New System.Drawing.Point(80, 110)
        Me.btn4.Size = New System.Drawing.Size(60, 60)
        Me.btn4.Text = ""
        Me.btn4.Name = "btn4"
        Me.btn4.TabIndex = 4
        Me.Controls.Add(Me.btn4)
        Me.btn5.Location = New System.Drawing.Point(140, 110)
        Me.btn5.Size = New System.Drawing.Size(60, 60)
        Me.btn5.Text = ""
        Me.btn5.Name = "btn5"
        Me.btn5.TabIndex = 5
        Me.Controls.Add(Me.btn5)
        Me.btn6.Location = New System.Drawing.Point(20, 170)
        Me.btn6.Size = New System.Drawing.Size(60, 60)
        Me.btn6.Text = ""
        Me.btn6.Name = "btn6"
        Me.btn6.TabIndex = 6
        Me.Controls.Add(Me.btn6)
        Me.btn7.Location = New System.Drawing.Point(80, 170)
        Me.btn7.Size = New System.Drawing.Size(60, 60)
        Me.btn7.Text = ""
        Me.btn7.Name = "btn7"
        Me.btn7.TabIndex = 7
        Me.Controls.Add(Me.btn7)
        Me.btn8.Location = New System.Drawing.Point(140, 170)
        Me.btn8.Size = New System.Drawing.Size(60, 60)
        Me.btn8.Text = ""
        Me.btn8.Name = "btn8"
        Me.btn8.TabIndex = 8
        Me.Controls.Add(Me.btn8)
        Me.lblStatus.Location = New System.Drawing.Point(20, 230)
        Me.lblStatus.Size = New System.Drawing.Size(180, 23)
        Me.lblStatus.Text = "Player X Turn"
        Me.lblStatus.Name = "lblStatus"
        Me.lblStatus.TabIndex = 9
        Me.Controls.Add(Me.lblStatus)
        Me.btnReset.Location = New System.Drawing.Point(20, 260)
        Me.btnReset.Size = New System.Drawing.Size(180, 40)
        Me.btnReset.Text = "Reset Game"
        Me.btnReset.Name = "btnReset"
        Me.btnReset.TabIndex = 10
        Me.Controls.Add(Me.btnReset)
        Me.ClientSize = New System.Drawing.Size(220, 320)
        Me.Text = "Tic Tac Toe"
        Me.Name = "Form1"
        Me.ResumeLayout(False)
    End Sub
End Class
