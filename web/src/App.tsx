import './index.css';

import { CameraControls } from '@react-three/drei';
import { Canvas, ThreeElements, useFrame, useThree } from '@react-three/fiber';
import { useEffect, useRef, useState } from 'react';
import { DoubleSide, Mesh, Vector3 } from 'three';

const WIDTH = 4;
const HEIGHT = WIDTH / 2;

function HalfSphere(props: ThreeElements['mesh']) {
  const mesh = useRef<Mesh>(null!);
  return (
    <mesh {...props} ref={mesh} position={[0, 0, 0]}>
      <sphereGeometry args={[HEIGHT, 32, 32, 0, Math.PI * 2, Math.PI / 2, Math.PI / 2]} />
      <meshStandardMaterial color={'grey'} side={DoubleSide} />
    </mesh>
  );
}

function Plane(props: ThreeElements['mesh']) {
  const mesh = useRef<Mesh>(null!);
  useEffect(() => {
    mesh.current.rotation.x = Math.PI;
  }, []);
  return (
    <mesh {...props} ref={mesh} position={[0, -HEIGHT / 2, 0]} castShadow={true}>
      <planeGeometry args={[WIDTH, HEIGHT]} />
      <meshStandardMaterial color={'red'} side={DoubleSide} />
    </mesh>
  );
}

function CameraState() {
  const camera = useThree((state) => state.camera);
  camera.position.set(0, 5, 0);
  camera.lookAt(0, 0, 0);
  return null;
}

export function App() {
  return (
    <div id="canvas-container">
      <Canvas shadows={true}>
        <ambientLight args={[0xff0000]} intensity={0.1} />
        <pointLight position={[0, 10, -5]} intensity={0.5} />
        <CameraState />
        <CameraControls />
        <HalfSphere />
        <Plane />
      </Canvas>
    </div>
  );
}
