import QtQuick
import QtQuick.Layouts
import org.kde.plasma.core as PlasmaCore
import org.kde.plasma.plasmoid
import org.kde.kirigami as Kirigami

PlasmoidItem {
    id: root

    // Tell the panel how big we want to be
    Layout.preferredWidth: rowLayout.implicitWidth

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
            var bestLine = null;
            for (var k = 0; k < chosenSnapshot.lines.length; k++) {
                var line = chosenSnapshot.lines[k];
                if (line.type === "progress") {
                    var val = line.used !== undefined ? line.used : line.value;
                    var maxVal = line.limit !== undefined ? line.limit : line.max;
                    var valNum = parseFloat(val);
                    var maxNum = parseFloat(maxVal);
                    if (!isNaN(valNum) && !isNaN(maxNum) && maxNum > 0) {
                        var percent = Math.round((valNum / maxNum) * 100);
                        if (percent < 100) {
                            bestLine = { line: line, percent: percent };
                            break;
                        }
                        if (!bestLine) {
                            bestLine = { line: line, percent: percent };
                        }
                    }
                }
            }
            if (bestLine) {
                var name = chosenSnapshot.displayName || chosenSnapshot.providerId;
                if (name.toLowerCase() === "antigravity") {
                    name = "Agy";
                }
                statusText = name + ": " + bestLine.percent + "%";
                return;
            }
        }

        statusText = "OpenUsage";
    }

    function toggleWindow(x, y) {
        var xhr = new XMLHttpRequest();
        var url = "http://127.0.0.1:6736/v1/toggle-window";
        if (x !== undefined && y !== undefined) {
            url += "?x=" + Math.round(x) + "&y=" + Math.round(y);
        }
        xhr.open("POST", url);
        xhr.send();
    }

    // Toggle window on click
    MouseArea {
        anchors.fill: parent
        cursorShape: Qt.PointingHandCursor
        onClicked: (mouse) => {
            var globalPos = mapToGlobal(mouse.x, mouse.y);
            toggleWindow(globalPos.x, globalPos.y);
        }
    }

    RowLayout {
        id: rowLayout
        anchors.fill: parent
        spacing: 6

        // Tray Icon (Kirigami.Icon is the Plasma 6 standard)
        Kirigami.Icon {
            id: icon
            source: "preferences-system-network-sharing"
            Layout.preferredWidth: parent.height - 8
            Layout.preferredHeight: parent.height - 8
            Layout.alignment: Qt.AlignVCenter
        }

        // Tray Text (Dynamic Usage)
        Text {
            id: label
            text: statusText
            color: Kirigami.Theme.textColor
            font.pixelSize: 13
            font.weight: Font.DemiBold
            Layout.alignment: Qt.AlignVCenter
        }
    }
}
