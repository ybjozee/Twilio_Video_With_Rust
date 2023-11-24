import { Form, Input } from "antd";
import { getRoom, getRoomToken } from "../Api";
import { useEffect, useState } from "react";
import { connect, createLocalVideoTrack } from "twilio-video";
import { useNavigate, useParams } from "react-router-dom";
import useNotification from "../hooks/useNotification";
import ModalForm from "./ModalForm";

const JoinRoom = () => {
  const { id } = useParams();
  const [showForm, setShowForm] = useState(false);
  const [room, setRoom] = useState(null);
  const [contextHolder, showFailure, showSuccess] = useNotification();
  const navigate = useNavigate();

  useEffect(() => {
    const getRoomDetails = async () => {
      try {
        const room = await getRoom(id);
        setRoom(room);
        setShowForm(room.hasPasscode);
        if (!room.hasPasscode) {
          const { token } = await getRoomToken({
            name: room.name,
            identity: room.identity,
            passcode: null,
          });
          joinRoom(token, room.name);
        }
      } catch (e) {
        showFailure({ title: "Failure", message: e.message });
      }
    };
    getRoomDetails();
  }, []);

  const joinRoom = async (token, name) => {
    const twilioRoom = await connect(token, { name });
    const videoTrack = await createLocalVideoTrack();
    const videoContainer = document.getElementById("remote-media");
    showSuccess({
      message: `Successfully joined a Room: ${twilioRoom}`,
    });
    videoContainer.appendChild(videoTrack.attach());
    twilioRoom.on("participantConnected", (participant) => {
      showSuccess({
        message: `A remote participant connected: ${participant.identity}`,
      });
      participant.tracks.forEach((publication) => {
        if (publication.isSubscribed) {
          const track = publication.track;
          videoContainer.appendChild(track.attach());
        }
      });
      participant.on("trackSubscribed", (track) => {
        videoContainer.appendChild(track.attach());
      });
    });
    twilioRoom.participants.forEach((participant) => {
      participant.tracks.forEach((publication) => {
        if (publication.track) {
          videoContainer.appendChild(publication.track.attach());
        }
      });
      participant.on("trackSubscribed", (track) => {
        videoContainer.appendChild(track.attach());
      });
    });
  };

  const handleFormSubmission = async (form) => {
    const { passcode } = await form.validateFields();
    try {
      const { token } = await getRoomToken({
        identity: room.identity,
        passcode,
      });
      form.resetFields();
      setShowForm(false);
      joinRoom(token, room.name);
    } catch (e) {
      showFailure({
        title: "Failed to join room",
        message: e.message,
      });
    }
  };

  const handleCancel = () => {
    navigate("/");
  };

  return (
    <>
      {contextHolder}
      <ModalForm
        title="Passcode required to join this room"
        isVisible={showForm}
        handleFormSubmission={handleFormSubmission}
        handleCancel={handleCancel}
      >
        <Form.Item
          name="passcode"
          label="Passcode"
          rules={[
            {
              required: true,
              message: "Please provide the room passcode",
            },
          ]}
        >
          <Input type="password" />
        </Form.Item>
      </ModalForm>
      <div id="remote-media"></div>
    </>
  );
};

export default JoinRoom;
