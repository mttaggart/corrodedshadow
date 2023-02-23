rule corroded_shadow {
   meta:
      description = "corrodedshadow.exe"
      author = "Michael Taggart <mtaggart@taggart-tech.com>"
      reference = "https://github.com/mttaggart/corrodedshadow"
      date = "2023-02-22"
   strings:
      $x1 = "CoInitializeEx" ascii
      $x2 = "CoInitializeSecurity" ascii
      $x3 = "CreateVssBackupComponentsInternal" ascii
      $x4 = 
   condition:
      filesize > 170KB and
      all of ($x*)
}