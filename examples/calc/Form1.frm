VERSION 5.00
Begin VB.Form Form1
   Caption         =   "Calculator"
   ClientHeight    =   280
   ClientLeft      =   60
   ClientTop       =   345
   ClientWidth     =   340
   LinkTopic       =   "Form1"
   ScaleHeight     =   280
   ScaleWidth      =   340
   StartUpPosition =   3  'Windows Default
   Begin VB.CommandButton btn1 
      Caption         =   "1"
      Height          =   30
      Left            =   40
      TabIndex        =   0
      Top             =   100
      Width           =   60
   End
   Begin VB.CommandButton btn2 
      Caption         =   "2"
      Height          =   30
      Left            =   110
      TabIndex        =   0
      Top             =   100
      Width           =   50
   End
   Begin VB.CommandButton btn3 
      Caption         =   "3"
      Height          =   30
      Left            =   170
      TabIndex        =   0
      Top             =   100
      Width           =   60
   End
   Begin VB.CommandButton btn4 
      Caption         =   "4"
      Height          =   30
      Left            =   40
      TabIndex        =   0
      Top             =   140
      Width           =   60
   End
   Begin VB.CommandButton btn5 
      Caption         =   "5"
      Height          =   30
      Left            =   110
      TabIndex        =   0
      Top             =   140
      Width           =   50
   End
   Begin VB.CommandButton btn6 
      Caption         =   "6"
      Height          =   30
      Left            =   170
      TabIndex        =   0
      Top             =   140
      Width           =   60
   End
   Begin VB.CommandButton btn7 
      Caption         =   "7"
      Height          =   30
      Left            =   40
      TabIndex        =   0
      Top             =   180
      Width           =   60
   End
   Begin VB.CommandButton btn8 
      Caption         =   "8"
      Height          =   30
      Left            =   110
      TabIndex        =   0
      Top             =   180
      Width           =   50
   End
   Begin VB.CommandButton btn9 
      Caption         =   "9"
      Height          =   30
      Left            =   170
      TabIndex        =   0
      Top             =   180
      Width           =   60
   End
   Begin VB.CommandButton btn0 
      Caption         =   "0"
      Height          =   30
      Left            =   40
      TabIndex        =   0
      Top             =   220
      Width           =   60
   End
   Begin VB.CommandButton btnDot 
      Caption         =   "."
      Height          =   30
      Left            =   110
      TabIndex        =   0
      Top             =   220
      Width           =   50
   End
   Begin VB.CommandButton btnPlus 
      Caption         =   "+"
      Height          =   30
      Left            =   240
      TabIndex        =   0
      Top             =   100
      Width           =   50
   End
   Begin VB.CommandButton btnMinus 
      Caption         =   "-"
      Height          =   30
      Left            =   240
      TabIndex        =   0
      Top             =   140
      Width           =   50
   End
   Begin VB.CommandButton btnDiv 
      Caption         =   "/"
      Height          =   30
      Left            =   240
      TabIndex        =   0
      Top             =   180
      Width           =   50
   End
   Begin VB.CommandButton btnTimes 
      Caption         =   "X"
      Height          =   30
      Left            =   240
      TabIndex        =   0
      Top             =   220
      Width           =   50
   End
   Begin VB.CommandButton btnEquals 
      Caption         =   "="
      Height          =   30
      Left            =   170
      TabIndex        =   0
      Top             =   220
      Width           =   60
   End
   Begin VB.TextBox txtCalc 
      Text            =   ""
      Height          =   20
      Left            =   40
      TabIndex        =   0
      Top             =   60
      Width           =   190
   End
   Begin VB.CommandButton btnClearDisplay 
      Caption         =   "C"
      Height          =   30
      Left            =   240
      TabIndex        =   0
      Top             =   60
      Width           =   50
   End
End
Attribute VB_Name = "Form1"
Attribute VB_GlobalNameSpace = False
Attribute VB_Creatable = False
Attribute VB_PredeclaredId = True
Attribute VB_Exposed = False
' Code file: Code1
Dim dblLastNum As Double    ' Stores the first number
Dim strOp As String         ' Stores the operator (+, -, /, *)
Dim blnClearDisplay As Boolean ' Flag to clear text when a new number is typed
Private Sub AddDigit(strDigit As String)
    If blnClearDisplay Then
        txtCalc.Text = strDigit
        blnClearDisplay = False
    Else
        txtCalc.Text = txtCalc.Text & strDigit
    End If
End Sub

Private Sub btn0_Click()
 AddDigit "0"
End Sub
Private Sub btn1_Click(): AddDigit "1": End Sub
Private Sub btn2_Click(): AddDigit "2": End Sub
Private Sub btn3_Click(): AddDigit "3": End Sub
Private Sub btn4_Click(): AddDigit "4": End Sub
Private Sub btn5_Click(): AddDigit "5": End Sub
Private Sub btn6_Click(): AddDigit "6": End Sub
Private Sub btn7_Click(): AddDigit "7": End Sub
Private Sub btn8_Click(): AddDigit "8": End Sub
Private Sub btn9_Click(): AddDigit "9": End Sub

Private Sub SetOperator(Op As String)
    dblLastNum = Val(txtCalc.Text)
    strOp = Op
    blnClearDisplay = True
End Sub

Private Sub btnPlus_Click():  SetOperator "+": End Sub
Private Sub btnMinus_Click(): SetOperator "-": End Sub
Private Sub btnTimes_Click(): SetOperator "*": End Sub
Private Sub btnDiv_Click():   SetOperator "/": End Sub

Private Sub btnEquals_Click()
    Dim dblCurrentNum As Double
    dblCurrentNum = Val(txtCalc.Text)

    Select Case strOp
        Case "+"
            txtCalc.Text = dblLastNum + dblCurrentNum
        Case "-"
            txtCalc.Text = dblLastNum - dblCurrentNum
        Case "*"
            txtCalc.Text = dblLastNum * dblCurrentNum
        Case "/"
            If dblCurrentNum <> 0 Then
                txtCalc.Text = dblLastNum / dblCurrentNum
            Else
                MsgBox "Error: Division by zero", vbCritical
            End If
    End Select
    
    ' Reset state so the next number typed starts fresh
    blnClearDisplay = True
End Sub
Private Sub btnDot_Click()
    ' Only add a dot if there isn't one already
    If InStr(txtCalc.Text, ".") = 0 Then
        txtCalc.Text = txtCalc.Text & "."
    End If
End Sub


Private Sub btn10_Click()
    ' TODO: Add your code here
    MsgBox "Hello"
End Sub
