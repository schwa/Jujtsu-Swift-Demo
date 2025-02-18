import SwiftUI
import jj_api
import UniformTypeIdentifiers

struct ContentView: View {
    @State
    var isFileImporterPresented: Bool = false

    @State
    var changes: [Change] = []

    var body: some View {
        List(Array(changes.enumerated()), id: \.0) { index in
            let change = self.changes[index.0]
            let changeId = change.changeId()
            HStack {
                Image(systemName: "circle")
                VStack(alignment: .leading) {
                    HStack {
                        HStack(spacing: 0) {
                            Text("\(changeId.shortestId())").bold()
                                .foregroundColor(Color.pink)
                            Text("\(changeId.id().trimmingPrefix(changeId.shortestId()))")
                        }
                        .monospaced()
                        Text("\(change.author().name())")
                    }
                    if change.description().isEmpty {
                        Text("no description set").italic()
                    }
                    else {
                        Text(change.description())
                    }
                }
            }
        }
        .toolbar {
            Button("Choose…") {
                isFileImporterPresented = true
            }
            .fileImporter(isPresented: $isFileImporterPresented, allowedContentTypes: [.directory]) { result in
                if case let .success(url) = result {
                    let path = url.path
                    let repo = repoFromPath(path: path)
                    changes = repo.log()
                }
            }
        }
        .navigationTitle("Swift Jujutsu")
    }
}
