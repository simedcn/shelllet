//import * as Test from "./index.js";
//import "shelllet/v0.1.0/common.let.dll"

console.log(`Hello World!`)
console.log($GOPATH)

// var hook = win.mouseHookStart((msg, r, pt )=>{
//     console.log(r);
//     console.log(pt.x);
//     console.log(pt.y);

//     if (msg == win.msg.mButtonDown) {
//         win.mouseHookStop(hook)
//         win.postQuitMessage();
//     }
// })

// win.runMessageLoop()


//win.simulateInput("CTRL + A")

//dir.mkAll("C:/Users/Admin/source/repos/gtktest/x64/Debug/share/themes/Windows10/gtk-3.0/")

//var ini = Ini.load("C:\\Windows\\System32\\wbem\\Performance\\WmiApRpl.ini")

//console.log(ini.sectionNames())

//console.log(win.keyIsDown(win.key.Shift))

//console.log(windows.keyName(windows.key.Ctrl))


// var hook = win.keyStartHook((msg, vk, sc) => {
//     if (msg == win.msg.KeyUp) {


//         console.log(vk);
//         console.log(sc);

//         console.log(win.keyName(vk))

//         if (vk == win.key.LCtrl) {
//             win.keyStopHook(hook)
//             win.postQuitMessage();
//         }
//     }
// })

// win.runMessageLoop()
//
// let ok = false
//
// let dwProcessId = 0
// windows.enumProcess((entry) => {
//     if (entry.exeFile == "shelllet.exe") {
//         dwProcessId = entry.processID;
//         return false;
//     }
// })
//
// windows.enumWindows((wnd) => {
//     if (!wnd) {
//         return true;
//     }
//     let processId = wnd.getProcessID();
//
//     // console.log(processId, wnd.getTitle());
//     if (processId == dwProcessId && !wnd.getOwner()) {
//
//         if (wnd.isTopLevel()) {
//             wnd.show(windows.cmdshow.SW_SHOWNORMAL);
//             wnd.foreground();
//             ok = true;
//             return false;
//         }
//
//     }
//
// });
// (()=>{return ok})()

// var pid = windows.getCurrentProcessID();

// windows.adjustPrivilege(pid, windows.pri.debug, true)

// windows.enumModule(620, (entry)=>{
//     console.log(JSON.stringify(entry))
// })


//runAs("test", "123", "C:\\Program Files (x86)\\FastStone Capture\\FSCapture.exe")

//run("C:\\Program Files (x86)\\Internet Download Manager\\IDMan.exe")

//shell("C:\\Program Files (x86)\\Internet Download Manager\\IDMan.exe")

// var k = windows.regCreateKey(windows.reg.CURRENT_USER, "Software\\Licenses\\version");
// k.close();


//windows.regDelKey(windows.reg.CURRENT_USER, "Software\\Classes\\Wow6432Node\\CLSID\\{79873CC5-3951-43ED-BDF9-D8759474B6FD}");


//start("D:\\Program Files (x86)\\Internet Download Manager\\IDMan.exe");

// windows.enumThread(0, (entry)=>{
//     console.log(JSON.stringify(entry))
// })

//console.log(Test.add(3, 3))

//console.log(Object.keys(FileSystem))
//console.log(Object.keys(FileSystem.Path))
//console.log(FileSystem.Path.userDocuments)

let id = windows.getCurrentThreadID();

windows.enumDesktopWindow(windows.getThreadDesktop(id), (window) => {
    if (window) {
        console.log(window.getTitle())
    }    
})
