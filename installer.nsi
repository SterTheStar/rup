!include "MUI2.nsh"

Name "rup 1.0.3"
OutFile "rup-installer-1.0.3.exe"
InstallDir "$PROGRAMFILES\rup"
InstallDirRegKey HKLM "Software\rup" ""
!insertmacro MUI_PAGE_LICENSE "LICENSE"
!insertmacro MUI_PAGE_DIRECTORY
!insertmacro MUI_PAGE_INSTFILES
!insertmacro MUI_UNPAGE_CONFIRM
!insertmacro MUI_UNPAGE_INSTFILES
!insertmacro MUI_LANGUAGE "English"

Section "Main"
  SetOutPath $INSTDIR
  File "builds\rup.exe"
  WriteRegStr HKLM "Software\rup" "" $INSTDIR
  WriteUninstaller "$INSTDIR\uninstall.exe"
  ; Add to PATH
  ReadRegStr $R0 HKLM "SYSTEM\CurrentControlSet\Control\Session Manager\Environment" "PATH"
  StrCmp $R0 "" 0 +2
  WriteRegExpandStr HKLM "SYSTEM\CurrentControlSet\Control\Session Manager\Environment" "PATH" "$INSTDIR"
  GoTo +3
  WriteRegExpandStr HKLM "SYSTEM\CurrentControlSet\Control\Session Manager\Environment" "PATH" "$R0;$INSTDIR"
  SendMessage ${HWND_BROADCAST} ${WM_WININICHANGE} 0 "STR:Environment" /TIMEOUT=5000
SectionEnd

Section "Uninstall"
  Delete "$INSTDIR\rup.exe"
  Delete "$INSTDIR\uninstall.exe"
  RMDir "$INSTDIR"
  DeleteRegKey HKLM "Software\rup"
  ; Remove from PATH
  ReadRegStr $R0 HKLM "SYSTEM\CurrentControlSet\Control\Session Manager\Environment" "PATH"
  StrCmp $R0 "" 0 +2
  GoTo +4
  Push "$INSTDIR"
  Call un.RemoveFromPath
  Pop $R0
  StrCmp $R0 "0" 0 +2
  WriteRegExpandStr HKLM "SYSTEM\CurrentControlSet\Control\Session Manager\Environment" "PATH" "$R0"
  SendMessage ${HWND_BROADCAST} ${WM_WININICHANGE} 0 "STR:Environment" /TIMEOUT=5000
SectionEnd

Function un.RemoveFromPath
  Exch $0
  Push $1
  Push $2
  Push $3
  Push $4
  Push $5
  Push $6

  ReadRegStr $1 HKLM "SYSTEM\CurrentControlSet\Control\Session Manager\Environment" "PATH"
  StrCpy $5 $1 1 -1
  StrCmp $5 ";" 0 +2
  StrCpy $1 $1 -1
  StrCpy $0 "$0;"
  Push $0
  Push $1
  Call un.StrStr
  Pop $2
  StrCmp $2 "" 0 +2
  StrCpy $3 $1
  GoTo +5
  StrCpy $3 $1 $2
  StrCpy $4 $2 "" 1
  StrCpy $4 $4 "" 1
  StrCpy $3 "$3$4"
  StrCpy $5 $3 1 -1
  StrCmp $5 ";" 0 +2
  StrCpy $3 $3 -1
  StrCpy $5 $3 1
  StrCmp $5 ";" 0 +2
  StrCpy $3 $3 "" 1

  WriteRegExpandStr HKLM "SYSTEM\CurrentControlSet\Control\Session Manager\Environment" "PATH" "$3"
  SendMessage ${HWND_BROADCAST} ${WM_WININICHANGE} 0 "STR:Environment" /TIMEOUT=5000

  Pop $6
  Pop $5
  Pop $4
  Pop $3
  Pop $2
  Pop $1
  Pop $0
FunctionEnd

Function un.StrStr
  Exch $R1
  Exch
  Exch $R2
  Push $R3
  Push $R4
  Push $R5
  StrLen $R3 $R1
  StrCpy $R4 0
  StrLen $R5 $R2
  IntCmp $R5 0 0 0 +3
  StrCpy $R1 ""
  Goto +7
  loop:
    StrCpy $R5 $R2 $R3 $R4
    StrCmp $R5 $R1 done
    IntCmp $R4 $R5 +3
    IntOp $R4 $R4 + 1
    Goto loop
  StrCpy $R1 ""
  done:
  Pop $R5
  Pop $R4
  Pop $R3
  Exch $R2
  Exch
  Exch $R1
FunctionEnd