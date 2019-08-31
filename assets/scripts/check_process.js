let ok = false

let dwProcessId = 0
windows.enumProcess((entry) => {
    if (entry.exeFile == "shelllet.exe") {
        dwProcessId = entry.processID;
        return false;
    }
})

windows.enumWindows((wnd) => {
    if (!wnd) {
        return true;
    }
    let processId = wnd.getProcessID();

    // console.log(processId, wnd.getTitle());
    if (processId == dwProcessId && !wnd.getOwner()) {
		wnd.show(windows.sw.SHOWNORMAL);
		wnd.foreground();
		ok = true;
		return false;
    }

});
(()=>{return ok})()