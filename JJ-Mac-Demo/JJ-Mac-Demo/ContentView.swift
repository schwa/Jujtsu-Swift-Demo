//
//  ContentView.swift
//  JJ-Mac-Demo
//
//  Created by Jonathan Wight on 2/17/25.
//

import SwiftUI
import jj_api

struct ContentView: View {
    var body: some View {
        VStack {
            Image(systemName: "globe")
                .imageScale(.large)
                .foregroundStyle(.tint)
            Text("Hello, world!")
        }
        .padding()
        .task {
            let repo = repoFromPath(path: "")
            repo.log()
        }
    }
}

#Preview {
    ContentView()
}
