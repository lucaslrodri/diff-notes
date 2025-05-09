interface Params {
    alt?: boolean;
    shift?: boolean;
    ctrl?: boolean;
    code: string;
    callback?: () => void;
}

export const shortcut = (node:HTMLElement, params: Params) => {
    let handler: (e: KeyboardEvent) => void;
    const removeHandler = () => window.removeEventListener('keydown', handler), setHandler = () => {
        removeHandler();
        if (!params)
            return;
        handler = (e: KeyboardEvent) => {
            if ((!!params.alt != e.altKey) ||
                (!!params.shift != e.shiftKey) ||
                (!!params.ctrl != (e.ctrlKey || e.metaKey)) ||
                params.code != e.code)
                return;
            e.preventDefault();
            params.callback ? params.callback() : node.click();
        };
        window.addEventListener('keydown', handler);
    };
    setHandler();
    return {
        update: setHandler,
        destroy: removeHandler,
    };
};
