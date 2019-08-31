;Include Modern UI

  !include "MUI2.nsh"
  !include "x64.nsh"

  Unicode true

;General
  !define VERSION "0.4.0"
  !define PRODUCT_NAME "shelllet ${VERSION}"
  !define APPNAME "shelllet"
  !define MUI_FOLDER "shelllet.com"
  !define COMPANYNAME "shelllet.com"
  !define HELPURL "http://shelllet.com"
  !define MUI_FINISHPAGE_NOAUTOCLOSE
  !define MUI_FINISHPAGE_RUN "$INSTDIR\${APPNAME}.exe"
  !define MUI_FINISHPAGE_RUN_CHECKED
  !define MUI_FINISHPAGE_RUN_TEXT "运行${PRODUCT_NAME}"
  !define MUI_FINISHPAGE_RUN_FUNCTION "LaunchLink"


  Name "${PRODUCT_NAME}"
  # Icon "${NSISDIR}\Contrib\Graphics\Icons\orange-install.ico"
  # UninstallIcon "${NSISDIR}\Contrib\Graphics\Icons\win-uninstall.ico"

  OutFile "${APPNAME}-${VERSION}-setup.x64.exe"

  ;Default installation folder
  InstallDir "$PROGRAMFILES64\${MUI_FOLDER}"

  ;Request application privileges for Windows Vista
  RequestExecutionLevel admin

;--------------------------------
;Variables

  Var StartMenuFolder

;--------------------------------
;Interface Configuration
  !define MUI_ICON "..\favicon.ico"
  !define MUI_UNICON "${NSISDIR}\Contrib\Graphics\Icons\orange-uninstall-nsis.ico"
  !define MUI_HEADERIMAGE
  !define MUI_HEADERIMAGE_BITMAP "${NSISDIR}\Contrib\Graphics\Header\nsis3-branding.bmp"
  !define MUI_WELCOMEFINISHPAGE_BITMAP "${NSISDIR}\Contrib\Graphics\Wizard\nsis3-branding.bmp"
  !define MUI_UNWELCOMEFINISHPAGE_BITMAP "${NSISDIR}\Contrib\Graphics\Wizard\nsis3-branding.bmp"
  !define MUI_ABORTWARNING

;Pages

  !insertmacro MUI_PAGE_COMPONENTS
  !insertmacro MUI_PAGE_DIRECTORY

  ;Start Menu Folder Page Configuration
  !define MUI_STARTMENUPAGE_REGISTRY_ROOT "HKCU"
  !define MUI_STARTMENUPAGE_REGISTRY_KEY "Software\${MUI_FOLDER}"
  !define MUI_STARTMENUPAGE_REGISTRY_VALUENAME "${MUI_FOLDER}"

  !insertmacro MUI_PAGE_STARTMENU Application $StartMenuFolder

  !insertmacro MUI_PAGE_INSTFILES
  !insertmacro MUI_PAGE_FINISH

  !insertmacro MUI_UNPAGE_COMPONENTS
  !insertmacro MUI_UNPAGE_WELCOME
  !insertmacro MUI_UNPAGE_CONFIRM

  !insertmacro MUI_UNPAGE_INSTFILES
  !insertmacro MUI_UNPAGE_FINISH
;--------------------------------
;Languages

  !insertmacro MUI_LANGUAGE "SimpChinese"

;--------------------------------
;Installer Sections

Section "安装文件" MAIN_FILES
    SetOutPath "$INSTDIR"
    ; Qt
    File C:\msys64\mingw64\bin\Qt5Core.dll
    File C:\msys64\mingw64\bin\Qt5Gui.dll
    File C:\msys64\mingw64\bin\Qt5Widgets.dll
    File C:\msys64\mingw64\bin\Qt5Svg.dll
    File C:\msys64\mingw64\bin\Qt5Qml.dll

    ; mingw64
    File C:\msys64\mingw64\bin\libgcc_s_seh-1.dll
    File C:\msys64\mingw64\bin\libwinpthread-1.dll
    File C:\msys64\mingw64\bin\zlib1.dll
    File C:\msys64\mingw64\bin\libzstd.dll
    File C:\msys64\mingw64\bin\libwebpdemux-2.dll
    File C:\msys64\mingw64\bin\libwebp-7.dll
    File C:\msys64\mingw64\bin\libtiff-5.dll
    File C:\msys64\mingw64\bin\libstdc++-6.dll
    File C:\msys64\mingw64\bin\libpng16-16.dll
    File C:\msys64\mingw64\bin\libpcre2-16-0.dll
    File C:\msys64\mingw64\bin\libpcre-1.dll
    File C:\msys64\mingw64\bin\liblzma-5.dll
    File C:\msys64\mingw64\bin\libjpeg-8.dll
    File C:\msys64\mingw64\bin\libjasper-4.dll
    File C:\msys64\mingw64\bin\libintl-8.dll
    File C:\msys64\mingw64\bin\libicuuc64.dll
    File C:\msys64\mingw64\bin\libicuin64.dll
    File C:\msys64\mingw64\bin\libicudt64.dll
    File C:\msys64\mingw64\bin\libiconv-2.dll
    File C:\msys64\mingw64\bin\libharfbuzz-0.dll
    File C:\msys64\mingw64\bin\libgraphite2.dll
    File C:\msys64\mingw64\bin\libglib-2.0-0.dll
    File C:\msys64\mingw64\bin\libfreetype-6.dll
    File C:\msys64\mingw64\bin\libdouble-conversion.dll
    File C:\msys64\mingw64\bin\libbz2-1.dll

    ; opencv
    File C:\opencv\build\bin\libopencv_videoio410.dll
    File C:\opencv\build\bin\libopencv_video410.dll
    File C:\opencv\build\bin\libopencv_objdetect410.dll
    File C:\opencv\build\bin\libopencv_imgproc410.dll
    File C:\opencv\build\bin\libopencv_imgcodecs410.dll
    File C:\opencv\build\bin\libopencv_highgui410.dll
    File C:\opencv\build\bin\libopencv_flann410.dll
    File C:\opencv\build\bin\libopencv_features2d410.dll
    File C:\opencv\build\bin\libopencv_dnn410.dll
    File C:\opencv\build\bin\libopencv_core410.dll
    File C:\opencv\build\bin\libopencv_calib3d410.dll
    ; v8
    File C:\v8\v8\out.gn\x64.release\v8.dll
    File C:\v8\v8\out.gn\x64.release\v8_libbase.dll
    File C:\v8\v8\out.gn\x64.release\v8_libplatform.dll

    File ..\bin\v8_c.dll
    File ..\bin\shelllet.exe
    File ..\bin\let.exe

    SetOutPath "$INSTDIR\plugins\styles"
    File C:\msys64\mingw64\share\qt5\plugins\styles\qwindowsvistastyle.dll

    SetOutPath "$INSTDIR\plugins\platforms"
    File C:\msys64\mingw64\share\qt5\plugins\platforms\qwindows.dll

    SetOutPath "$INSTDIR\plugins\imageformats"
    File C:\msys64\mingw64\share\qt5\plugins\imageformats\qwebp.dll
    File C:\msys64\mingw64\share\qt5\plugins\imageformats\qwbmp.dll
    File C:\msys64\mingw64\share\qt5\plugins\imageformats\qtiff.dll
    File C:\msys64\mingw64\share\qt5\plugins\imageformats\qtga.dll
    File C:\msys64\mingw64\share\qt5\plugins\imageformats\qsvg.dll
    File C:\msys64\mingw64\share\qt5\plugins\imageformats\qjpeg.dll
    File C:\msys64\mingw64\share\qt5\plugins\imageformats\qjp2.dll
    File C:\msys64\mingw64\share\qt5\plugins\imageformats\qico.dll
    File C:\msys64\mingw64\share\qt5\plugins\imageformats\qgif.dll
	
	SetOutPath "$INSTDIR\assets"
	File /r ..\assets

    ;create desktop shortcut
    CreateShortCut "$DESKTOP\${PRODUCT_NAME}.lnk" "$INSTDIR\${APPNAME}.exe" ""
   
    !insertmacro MUI_STARTMENU_WRITE_BEGIN Application
    
    ;create start-menu items
    CreateDirectory "$SMPROGRAMS\${PRODUCT_NAME}"
    CreateShortCut "$SMPROGRAMS\${PRODUCT_NAME}\Uninstall.lnk" "$INSTDIR\Uninstall.exe" "" "$INSTDIR\Uninstall.exe" 0
    CreateShortCut "$SMPROGRAMS\${PRODUCT_NAME}\${PRODUCT_NAME}.lnk" "$INSTDIR\${APPNAME}.exe" "" "$INSTDIR\${APPNAME}.exe" 0

    !insertmacro MUI_STARTMENU_WRITE_END

    ;Store installation folder
    WriteRegStr HKCU "Software\${PRODUCT_NAME}" "" $INSTDIR
    WriteRegStr HKLM "Software\Microsoft\Windows\CurrentVersion\Uninstall\${COMPANYNAME} ${APPNAME}" "DisplayName" "${PRODUCT_NAME}"
    WriteRegStr HKLM "Software\Microsoft\Windows\CurrentVersion\Uninstall\${COMPANYNAME} ${APPNAME}" "UninstallString" "$\"$INSTDIR\Uninstall.exe$\""
    WriteRegStr HKLM "Software\Microsoft\Windows\CurrentVersion\Uninstall\${COMPANYNAME} ${APPNAME}" "QuietUninstallString" "$\"$INSTDIR\Uninstall.exe$\" /S"
    WriteRegStr HKLM "Software\Microsoft\Windows\CurrentVersion\Uninstall\${COMPANYNAME} ${APPNAME}" "InstallLocation" "$\"$INSTDIR$\""
    WriteRegStr HKLM "Software\Microsoft\Windows\CurrentVersion\Uninstall\${COMPANYNAME} ${APPNAME}" "DisplayIcon" "$\"$INSTDIR\${APPNAME}.exe$\""
    WriteRegStr HKLM "Software\Microsoft\Windows\CurrentVersion\Uninstall\${COMPANYNAME} ${APPNAME}" "Publisher" "$\"${COMPANYNAME}$\""
    WriteRegStr HKLM "Software\Microsoft\Windows\CurrentVersion\Uninstall\${COMPANYNAME} ${APPNAME}" "HelpLink" "$\"${HELPURL}$\""
    
    ;Create uninstaller
    WriteUninstaller "$INSTDIR\Uninstall.exe"

SectionEnd


;Language strings
;LangString DESC_SecFiles ${LANG_ENGLISH} "basic component"
LangString DESC_MAIN_SECFILES ${LANG_SIMPCHINESE} "Main"

;Assign language strings to sections
!insertmacro MUI_FUNCTION_DESCRIPTION_BEGIN
  !insertmacro MUI_DESCRIPTION_TEXT ${MAIN_FILES} $(DESC_MAIN_SECFILES)
!insertmacro MUI_FUNCTION_DESCRIPTION_END



;Uninstaller Section
Section Un.Main UnMain

  ;ADD YOUR OWN FILES HERE...

  Delete "$INSTDIR\Uninstall.exe"

  RMDir /r "$INSTDIR"

  !insertmacro MUI_STARTMENU_GETFOLDER Application $StartMenuFolder

  Delete "$SMPROGRAMS\$StartMenuFolder\Uninstall.lnk"
  Delete "$DESKTOP\${PRODUCT_NAME}.lnk"
  RMDir "$SMPROGRAMS\$StartMenuFolder"

  DeleteRegKey /ifempty HKCU "Software\${PRODUCT_NAME}"
  DeleteRegKey HKLM "Software\Microsoft\Windows\CurrentVersion\Uninstall\${COMPANYNAME} ${APPNAME}"
SectionEnd

Section Un.Script

  SetShellVarContext current

  RMDir /r "$PROGRAMFILES64\scripts"

SectionEnd

Function LaunchLink
  ExecShell "" "$INSTDIR\${APPNAME}.exe"
FunctionEnd

ShowInstDetails show

ShowUnInstDetails show


Function .onInit
  # set section 'test' as selected and read-only
  IntOp $0 ${SF_SELECTED} | ${SF_RO}
  SectionSetFlags ${MAIN_FILES} $0
FunctionEnd

Function un.onInit
  # set section 'test' as selected and read-only
  IntOp $0 ${SF_SELECTED} | ${SF_RO}
  SectionSetFlags ${UnMain} $0
FunctionEnd