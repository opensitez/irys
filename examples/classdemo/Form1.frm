VERSION 5.00
Begin VB.Form Form1
   Caption         =   "Simple Class Demo"
   ClientHeight    =   480
   ClientLeft      =   60
   ClientTop       =   345
   ClientWidth     =   640
   LinkTopic       =   "Form1"
   ScaleHeight     =   480
   ScaleWidth      =   640
   StartUpPosition =   3  'Windows Default
   Begin VB.CommandButton btnClass 
      Caption         =   "Click To Instantiate"
      Height          =   30
      Left            =   10
      TabIndex        =   0
      Top             =   240
      Width           =   210
      BackColor       =   &H0000ECF5&
      ForeColor       =   &H002A170F&
      Font            =   "Segoe UI, 12px"
      Enabled         =   -1
      Visible         =   -1
   End
   Begin VB.Label lbl1 
      Caption         =   "Click on instantiate to create an instance of class person"
      Height          =   80
      Left            =   20
      TabIndex        =   0
      Top             =   90
      Width           =   180
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

Private Sub btnClass_Click()
        ' 1. Declare and Instantiate the class
        Dim user As New Person()

        ' 2. Set the property
        user.Name = "Alice"

        ' 3. Call the method
        user.Greet()
End Sub

