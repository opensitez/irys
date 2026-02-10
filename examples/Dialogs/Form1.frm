VERSION 5.00
Begin VB.Form Form1
   Caption         =   "Form1"
   ClientHeight    =   480
   ClientLeft      =   60
   ClientTop       =   345
   ClientWidth     =   640
   LinkTopic       =   "Form1"
   ScaleHeight     =   480
   ScaleWidth      =   640
   StartUpPosition =   3  'Windows Default
   Begin VB.CommandButton btnColor 
      Caption         =   "Color"
      Height          =   30
      Left            =   40
      TabIndex        =   0
      Top             =   90
      Width           =   120
      BackColor       =   &H00FCFAF8&
      ForeColor       =   &H002A170F&
      Font            =   "Segoe UI, 12px"
      Enabled         =   -1
      Visible         =   -1
   End
   Begin VB.CommandButton btnOpen 
      Caption         =   "Open"
      Height          =   30
      Left            =   40
      TabIndex        =   0
      Top             =   160
      Width           =   120
      BackColor       =   &H00FCFAF8&
      ForeColor       =   &H002A170F&
      Font            =   "Segoe UI, 12px"
      Enabled         =   -1
      Visible         =   -1
   End
   Begin VB.CommandButton btnInput 
      Caption         =   "Input"
      Height          =   30
      Left            =   40
      TabIndex        =   0
      Top             =   230
      Width           =   120
      BackColor       =   &H00FCFAF8&
      ForeColor       =   &H002A170F&
      Font            =   "Segoe UI, 12px"
      Enabled         =   -1
      Visible         =   -1
   End
   Begin VB.CommandButton btnFont 
      Caption         =   "Font"
      Height          =   30
      Left            =   180
      TabIndex        =   0
      Top             =   90
      Width           =   120
      BackColor       =   &H00FCFAF8&
      ForeColor       =   &H002A170F&
      Font            =   "Segoe UI, 12px"
      Enabled         =   -1
      Visible         =   -1
   End
   Begin VB.CommandButton btnSave 
      Caption         =   "Save"
      Height          =   30
      Left            =   180
      TabIndex        =   0
      Top             =   160
      Width           =   120
      BackColor       =   &H00FCFAF8&
      ForeColor       =   &H002A170F&
      Font            =   "Segoe UI, 12px"
      Enabled         =   -1
      Visible         =   -1
   End
   Begin VB.CommandButton btnMsg 
      Caption         =   "MsgBox"
      Height          =   30
      Left            =   180
      TabIndex        =   0
      Top             =   230
      Width           =   120
      BackColor       =   &H00FCFAF8&
      ForeColor       =   &H002A170F&
      Font            =   "Segoe UI, 12px"
      Enabled         =   -1
      Visible         =   -1
   End
End
Attribute VB_Name = "Form1"
Attribute VB_GlobalNameSpace = False
Attribute VB_Creatable = False
Attribute VB_PredeclaredId = True
Attribute VB_Exposed = False


Private Sub btnMsg_Click()
    ' Msg Dialog
    MsgBox "Hello"
End Sub

Private Sub btnFont_Click()
    ' Font Dialog
    Dim fontDlg As New FontDialog()
    If fontDlg.ShowDialog() Then
        txtBox.Font = fontDlg.Font
    End If
End Sub

Private Sub btnColor_Click()
    ' Color Dialog
    Dim colorDlg As New ColorDialog()
    If colorDlg.ShowDialog() Then
        txtBox.BackColor = colorDlg.Color
    End If
End Sub

Private Sub btnOpen_Click()
    ' Open File Dialog
    Dim openDlg As New OpenFileDialog()
    openDlg.Filter = "Text Files|*.txt|All Files|*.*"
    openDlg.Title = "Open File"
    If openDlg.ShowDialog() Then
        Dim filename As String = openDlg.FileName
        ' Process file...
    End If

End Sub

Private Sub btnSave_Click()
    ' Save File Dialog  
    Dim saveDlg As New SaveFileDialog()
    saveDlg.Filter = "VB Files|*.vb|All Files|*.*"
    If saveDlg.ShowDialog() Then
        ' Save to saveDlg.FileName
    End If
End Sub

Private Sub btnInput_Click()
    ' Input Box
    Dim name As String = InputBox("Enter your name:", "Name Entry", "Default")
End Sub







