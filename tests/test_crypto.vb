Imports System.Security.Cryptography
Imports System.Text

Module CryptoTest
    Sub Main()
        Console.WriteLine("--- Cryptography Verification ---")
        
        Dim input = "hello vybe"
        Dim bytes = Encoding.UTF8.GetBytes(input)
        
        ' 1. MD5
        Dim md5obj = MD5.Create()
        Dim hash1 = md5obj.ComputeHash(bytes)
        Dim hashStr1 = BitConverter.ToString(hash1).Replace("-", "").ToLower()
        Console.WriteLine("MD5: " & hashStr1)
        
        ' MD5 hash of "hello vybe" is 49793144063857d427fa1179cb10c4f8
        If hashStr1 = "49793144063857d427fa1179cb10c4f8" Then
            Console.WriteLine("MD5 Success")
        Else
            Console.WriteLine("MD5 Failure")
        End If
        
        ' 2. SHA256
        Dim sha = SHA256.Create()
        Dim hash2 = sha.ComputeHash(bytes)
        Dim hashStr2 = BitConverter.ToString(hash2).Replace("-", "").ToLower()
        Console.WriteLine("SHA256: " & hashStr2)
        
        ' SHA256 hash of "hello vybe" is 28da0c776097561858c89423c8309df38210134468f7637841cbad89ed62f689
        If hashStr2 = "28da0c776097561858c89423c8309df38210134468f7637841cbad89ed62f689" Then
            Console.WriteLine("SHA256 Success")
        Else
            Console.WriteLine("SHA256 Failure")
        End If
    End Sub
End Module
