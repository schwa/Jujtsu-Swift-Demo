import jj_api

let repo = repoFromPath(path: ".")
// let repo = Repo(path: ".")
let result = repo.log()
for change in result {
    print("[\(change.changeId().shortestId())]: \"\(change.description())\" \(change.author().name())")
}
