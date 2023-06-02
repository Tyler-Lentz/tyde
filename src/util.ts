export function getIndex(): number {
    let range = document.createRange();
    let sel = window.getSelection();
    return sel?.getRangeAt(0).startOffset || 0;
}

export function setIndex(indx: number, el: HTMLPreElement) {
    let range = document.createRange();
    let sel = window.getSelection();
    if (indx > el.innerText.length) {
        indx = el.innerText.length;
    }
    if (el.childNodes.length > 0) {
        range.setStart(el.childNodes[0], indx);
        range.collapse(true);
        sel?.removeAllRanges();
        sel?.addRange(range);
    }
}

export function arrowLeft(el: HTMLPreElement) {
    let range = document.createRange();
    let sel = window.getSelection();
    let start = sel?.getRangeAt(0).startOffset;
    if (start !== undefined && start > 0) {
        if (el.childNodes.length > 0) {
            range.setStart(el.childNodes[0], start - 1);
            range.collapse(true);
            sel?.removeAllRanges();
            sel?.addRange(range);
            el.focus();
        }
    }
}

export function arrowRight(el: HTMLPreElement) {
    let range = document.createRange();
    let sel = window.getSelection();
    let start = sel?.getRangeAt(0).startOffset;
    if (start !== undefined) {
        if (el.childNodes.length > 0) {
            range.setStart(el.childNodes[0], start + 1);
            range.collapse(true);
            sel?.removeAllRanges();
            sel?.addRange(range);
            el.focus();
        }
    }
}