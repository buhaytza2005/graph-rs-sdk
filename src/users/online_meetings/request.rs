// GENERATED CODE

use crate::api_default_imports::*;

resource_api_client!(OnlineMeetingsApiClient, OnlineMeetingsIdApiClient, ResourceIdentity::OnlineMeetings);

impl OnlineMeetingsApiClient {
	post!(
		doc: "Create onlineMeeting", 
		name: create_online_meetings,
		path: "/onlineMeetings",
		body: true
	);
	get!(
		doc: "Get onlineMeeting", 
		name: list_online_meetings,
		path: "/onlineMeetings"
	);
	get!(
		doc: "Get the number of the resource", 
		name: online_meetings,
		path: "/onlineMeetings/$count"
	);
	post!(
		doc: "Invoke action createOrGet", 
		name: create_or_get,
		path: "/onlineMeetings/createOrGet",
		body: true
	);
}

impl OnlineMeetingsIdApiClient {
	delete!(
		doc: "Delete onlineMeeting", 
		name: delete_online_meetings,
		path: "/onlineMeetings/{{RID}}"
	);
	get!(
		doc: "Get onlineMeeting", 
		name: get_online_meetings,
		path: "/onlineMeetings/{{RID}}"
	);
	patch!(
		doc: "Update onlineMeeting", 
		name: update_online_meetings,
		path: "/onlineMeetings/{{RID}}",
		body: true
	);
	post!(
		doc: "Create new navigation property to attendanceReports for users", 
		name: create_attendance_reports,
		path: "/onlineMeetings/{{RID}}/attendanceReports",
		body: true
	);
	get!(
		doc: "Get attendanceReports from users", 
		name: list_attendance_reports,
		path: "/onlineMeetings/{{RID}}/attendanceReports"
	);
	get!(
		doc: "Get the number of the resource", 
		name: attendance_reports,
		path: "/onlineMeetings/{{RID}}/attendanceReports/$count"
	);
	delete!(
		doc: "Delete navigation property attendanceReports for users", 
		name: delete_attendance_reports,
		path: "/onlineMeetings/{{RID}}/attendanceReports/{{id}}",
		params: meeting_attendance_report_id
	);
	get!(
		doc: "Get attendanceReports from users", 
		name: get_attendance_reports,
		path: "/onlineMeetings/{{RID}}/attendanceReports/{{id}}",
		params: meeting_attendance_report_id
	);
	patch!(
		doc: "Update the navigation property attendanceReports in users", 
		name: update_attendance_reports,
		path: "/onlineMeetings/{{RID}}/attendanceReports/{{id}}",
		body: true,
		params: meeting_attendance_report_id
	);
	post!(
		doc: "Create new navigation property to attendanceRecords for users", 
		name: create_attendance_records,
		path: "/onlineMeetings/{{RID}}/attendanceReports/{{id}}/attendanceRecords",
		body: true,
		params: meeting_attendance_report_id
	);
	get!(
		doc: "List attendanceRecords", 
		name: list_attendance_records,
		path: "/onlineMeetings/{{RID}}/attendanceReports/{{id}}/attendanceRecords",
		params: meeting_attendance_report_id
	);
	get!(
		doc: "Get the number of the resource", 
		name: attendance_records,
		path: "/onlineMeetings/{{RID}}/attendanceReports/{{id}}/attendanceRecords/$count",
		params: meeting_attendance_report_id
	);
	delete!(
		doc: "Delete navigation property attendanceRecords for users", 
		name: delete_attendance_records,
		path: "/onlineMeetings/{{RID}}/attendanceReports/{{id}}/attendanceRecords/{{id2}}",
		params: meeting_attendance_report_id, attendance_record_id
	);
	get!(
		doc: "Get attendanceRecords from users", 
		name: get_attendance_records,
		path: "/onlineMeetings/{{RID}}/attendanceReports/{{id}}/attendanceRecords/{{id2}}",
		params: meeting_attendance_report_id, attendance_record_id
	);
	patch!(
		doc: "Update the navigation property attendanceRecords in users", 
		name: update_attendance_records,
		path: "/onlineMeetings/{{RID}}/attendanceReports/{{id}}/attendanceRecords/{{id2}}",
		body: true,
		params: meeting_attendance_report_id, attendance_record_id
	);
	get!(
		doc: "Get attendeeReport for the navigation property onlineMeetings from users", 
		name: get_online_meetings_attendee_report,
		path: "/onlineMeetings/{{RID}}/attendeeReport"
	);
	put!(
		doc: "Update attendeeReport for the navigation property onlineMeetings in users", 
		name: update_online_meetings_attendee_report,
		path: "/onlineMeetings/{{RID}}/attendeeReport",
		body: true
	);
	get!(
		doc: "Invoke function getVirtualAppointmentJoinWebUrl", 
		name: get_virtual_appointment_join_web_url,
		path: "/onlineMeetings/{{RID}}/getVirtualAppointmentJoinWebUrl()"
	);
	post!(
		doc: "Create new navigation property to recordings for users", 
		name: create_recordings,
		path: "/onlineMeetings/{{RID}}/recordings",
		body: true
	);
	get!(
		doc: "Get callRecording", 
		name: list_recordings,
		path: "/onlineMeetings/{{RID}}/recordings"
	);
	get!(
		doc: "Get the number of the resource", 
		name: recordings,
		path: "/onlineMeetings/{{RID}}/recordings/$count"
	);
	delete!(
		doc: "Delete navigation property recordings for users", 
		name: delete_recordings,
		path: "/onlineMeetings/{{RID}}/recordings/{{id}}",
		params: call_recording_id
	);
	get!(
		doc: "Get callRecording", 
		name: get_recordings,
		path: "/onlineMeetings/{{RID}}/recordings/{{id}}",
		params: call_recording_id
	);
	patch!(
		doc: "Update the navigation property recordings in users", 
		name: update_recordings,
		path: "/onlineMeetings/{{RID}}/recordings/{{id}}",
		body: true,
		params: call_recording_id
	);
	get!(
		doc: "Get content for the navigation property recordings from users", 
		name: get_recordings_content,
		path: "/onlineMeetings/{{RID}}/recordings/{{id}}/content",
		params: call_recording_id
	);
	put!(
		doc: "Update content for the navigation property recordings in users", 
		name: update_recordings_content,
		path: "/onlineMeetings/{{RID}}/recordings/{{id}}/content",
		body: true,
		params: call_recording_id
	);
	post!(
		doc: "Create new navigation property to transcripts for users", 
		name: create_transcripts,
		path: "/onlineMeetings/{{RID}}/transcripts",
		body: true
	);
	get!(
		doc: "List transcripts", 
		name: list_transcripts,
		path: "/onlineMeetings/{{RID}}/transcripts"
	);
	get!(
		doc: "Get the number of the resource", 
		name: transcripts,
		path: "/onlineMeetings/{{RID}}/transcripts/$count"
	);
	delete!(
		doc: "Delete navigation property transcripts for users", 
		name: delete_transcripts,
		path: "/onlineMeetings/{{RID}}/transcripts/{{id}}",
		params: call_transcript_id
	);
	get!(
		doc: "Get callTranscript", 
		name: get_transcripts,
		path: "/onlineMeetings/{{RID}}/transcripts/{{id}}",
		params: call_transcript_id
	);
	patch!(
		doc: "Update the navigation property transcripts in users", 
		name: update_transcripts,
		path: "/onlineMeetings/{{RID}}/transcripts/{{id}}",
		body: true,
		params: call_transcript_id
	);
	get!(
		doc: "Get content for the navigation property transcripts from users", 
		name: get_transcripts_content,
		path: "/onlineMeetings/{{RID}}/transcripts/{{id}}/content",
		params: call_transcript_id
	);
	put!(
		doc: "Update content for the navigation property transcripts in users", 
		name: update_transcripts_content,
		path: "/onlineMeetings/{{RID}}/transcripts/{{id}}/content",
		body: true,
		params: call_transcript_id
	);
	get!(
		doc: "Get metadataContent for the navigation property transcripts from users", 
		name: get_transcripts_metadata_content,
		path: "/onlineMeetings/{{RID}}/transcripts/{{id}}/metadataContent",
		params: call_transcript_id
	);
	put!(
		doc: "Update metadataContent for the navigation property transcripts in users", 
		name: update_transcripts_metadata_content,
		path: "/onlineMeetings/{{RID}}/transcripts/{{id}}/metadataContent",
		body: true,
		params: call_transcript_id
	);
}
