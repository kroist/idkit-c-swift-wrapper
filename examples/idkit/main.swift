import Foundation

// We use a DispatchGroup to make the program wait until
// our async function finishes before exiting.
// You wouldn't need to do this in a long-running applications.
let group = DispatchGroup()
group.enter()

print("We're in Swift about to call our async Rust function.")
Task {
    let ipAddress = await get_url(
        "app_14d0217a5ec381effeb27d036b7c6c63",
        "rust-test"
    )
    print("Now we're in Swift again. IP address: \(ipAddress.toString())")

    group.leave()
}

group.wait()
