(function() {


//#region packages/website/input.ts
	document.addEventListener("alpine:init", () => {
		Alpine.store("todo", {
			items: [],
			add(item) {
				this.items.push(item);
			},
			remove(index) {
				this.items.splice(index, 1);
			}
		});
	});

//#endregion
})();