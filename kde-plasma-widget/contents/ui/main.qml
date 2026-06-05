import QtQuick 2.15
import QtQuick.Layouts 1.15
import org.kde.plasma.core 2.0 as PlasmaCore
import org.kde.plasma.plasmoid 2.0

Item {
    id: root

    // Tell the panel how big we want to be
    Plasmoid.preferredWidth: rowLayout.implicitWidth
    // Ensure height matches panel height
    Plasmoid.preferredHeight: Plasmoid.compactRepresentationHeight

    property string statusText: "Agy: --%"

    // Fetch data every 15 seconds
    Timer {
        id: refreshTimer
        interval: 15000
        running: true
        repeat: true
        triggeredOnStart: true
        onTriggered: {
            fetchUsageData()
        }
    }

    function fetchUsageData() {
        var xhr = new XMLHttpRequest();
        xhr.onreadystatechange = function() {
            if (xhr.readyState === XMLHttpRequest.DONE) {
                if (xhr.status === 200) {
                    try {
                        var data = JSON.parse(xhr.responseText);
                        updateWidgetState(data);
                    } catch (e) {
                        statusText = "Error";
                    }
                } else {
                    statusText = "Agy: --%";
                }
            }
        }
        xhr.open("GET", "http://127.0.0.1:6736/v1/usage");
        xhr.send();
    }

    function updateWidgetState(data) {
        if (!data || data.length === 0) {
            statusText = "OpenUsage";
            return;
        }

        // Search for Antigravity first as priority, then Claude or others
        var chosenSnapshot = null;
        for (var i = 0; i < data.length; i++) {
            var snapshot = data[i];
            if (snapshot.providerId === "antigravity") {
                chosenSnapshot = snapshot;
                break;
            }
        }

        // Fallback to first available snapshot with lines
        if (!chosenSnapshot) {
            for (var j = 0; j < data.length; j++) {
                if (data[j].lines && data[j].lines.length > 0) {
                    chosenSnapshot = data[j];
                    break;
                }
            }
        }

        if (chosenSnapshot && chosenSnapshot.lines && chosenSnapshot.lines.length > 0) {
            // Find the first progress line (which holds the primary metric)
            for (var k = 0; k < chosenSnapshot.lines.length; k++) {
                var line = chosenSnapshot.lines[k];
                if (line.type === "progress" || line.value !== undefined) {
                    var name = chosenSnapshot.displayName || chosenSnapshot.providerId;
                    if (name.toLowerCase() === "antigravity") {
                        name = "Agy";
                    }
                    var percent = Math.round((line.value / line.max) * 100);
                    statusText = name + ": " + percent + "%";
                    return;
                }
            }
        }

        statusText = "OpenUsage";
    }

    function toggleWindow() {
        var xhr = new XMLHttpRequest();
        xhr.open("POST", "http://127.0.0.1:6736/v1/toggle-window");
        xhr.send();
    }

    // Toggle window on click
    MouseArea {
        anchors.fill: parent
        cursorShape: Qt.PointingHandCursor
        onClicked: {
            toggleWindow()
        }
    }

    RowLayout {
        id: rowLayout
        anchors.fill: parent
        spacing: 6

        // Tray Icon
        PlasmaCore.IconItem {
            id: icon
            source: "preferences-system-network-sharing"
            Layout.width: parent.height - 8
            Layout.height: parent.height - 8
            Layout.alignment: Qt.AlignVCenter
        }

        // Tray Text (Dynamic Usage)
        Text {
            id: label
            text: statusText
            color: theme.textColor
            font.pixelSize: 13
            font.weight: Font.DemiBold
            Layout.alignment: Qt.AlignVCenter
        }
    }
}
