// arrow code taken from 
// https://stackoverflow.com/questions/17880821/javascript-simulate-key-events-into-textbox-input
export function arrowLeft(elm: HTMLTextAreaElement) {
    elm.selectionStart = elm.selectionEnd -= 1;
}

export function arrowRight(elm: HTMLTextAreaElement) {
    elm.selectionStart = elm.selectionEnd += 1;
}

export function arrowDown(elm: HTMLTextAreaElement) {
    var pos = elm.selectionEnd,
        prevLine = elm.value.lastIndexOf('\n', pos),
        nextLine = elm.value.indexOf('\n', pos + 1);
    if (nextLine === -1) return;
    pos = pos - prevLine;
    elm.selectionStart = elm.selectionEnd = nextLine + pos;
}

export function arrowUp(elm: HTMLTextAreaElement) {
    var pos = elm.selectionEnd,
        prevLine = elm.value.lastIndexOf('\n', pos),
        TwoBLine = elm.value.lastIndexOf('\n', prevLine - 1);
    if (prevLine === -1) return;
    pos = pos - prevLine;
    elm.selectionStart = elm.selectionEnd = TwoBLine + pos;
}

export function insertAtCursor(myField: HTMLTextAreaElement, myValue: string) {
    let startPos = myField.selectionStart;
    let endPos = myField.selectionEnd;
    myField.value = myField.value.substring(0, startPos)
        + myValue
        + myField.value.substring(endPos, myField.value.length);
}