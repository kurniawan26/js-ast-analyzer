// Test file for null safety analysis

// 1. Chained property access without null check
function getUserData(user) {
    // Unsafe: accessing nested properties without validation
    const name = user.profile.name; // Could crash if user.profile is null
    const email = user.profile.contact.email; // Even worse - nested twice

    // Better: use optional chaining
    const safeName = user?.profile?.name;
    const safeEmail = user?.profile?.contact?.email;

    return { name, email };
}

// 2. Array access without validation
function processItems(items) {
    // Unsafe: assuming array has elements
    const first = items[0]; // Could be undefined
    const second = items[1]; // Could be undefined

    // Better: check length first
    const safeFirst = items && items.length > 0 ? items[0] : null;
    const safeSecond = items && items.length > 1 ? items[1] : null;

    return { first, second };
}

// 3. Array methods without null check
function displayUsers(users) {
    // Unsafe: if users is null/undefined, this will crash
    const names = users.map(u => u.name);
    const active = users.filter(u => u.active);

    // Better: check first
    const safeNames = users ? users.map(u => u.name) : [];
    const safeActive = users ? users.filter(u => u.active) : [];

    return { names, active };
}

// 4. Destructuring without defaults
function parseConfig(config) {
    // Unsafe: if config.options is undefined, these will fail
    const { debug, timeout } = config.options;

    // Better: provide defaults
    const { debug = false, timeout = 5000 } = config.options || {};

    return { debug, timeout };
}

// 5. Array destructuring without defaults
function getFirstItem(items) {
    // Unsafe: if items is empty, first will be undefined
    const [first] = items;

    // Better: provide default
    const [firstOrDefault = null] = items || [];

    return first;
}
