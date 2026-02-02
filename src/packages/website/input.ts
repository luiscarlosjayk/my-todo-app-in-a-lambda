document.addEventListener("alpine:init", () => {
    interface Todo {
        id: string;
        text: string;
        completed: boolean;
    }
    /* @ts-ignore */
    Alpine.store("todo", {
        items: <Todo[]>[],
        add(item: Todo) {
            this.items.push(item);
        },
        remove(index: number) {
            this.items.splice(index, 1);
        },
    });
});
