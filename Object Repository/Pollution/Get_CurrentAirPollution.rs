<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Get_CurrentAirPollution</name>
   <tag></tag>
   <elementGuidId>99ec63f9-d83f-4361-9423-02b197ad2eb0</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <smartLocatorEnabled>false</smartLocatorEnabled>
   <useRalativeImagePath>false</useRalativeImagePath>
   <autoUpdateContent>true</autoUpdateContent>
   <connectionTimeout>0</connectionTimeout>
   <followRedirects>true</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent></httpBodyContent>
   <httpBodyType></httpBodyType>
   <katalonVersion>10.2.0</katalonVersion>
   <maxResponseSize>0</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <path></path>
   <restRequestMethod>GET</restRequestMethod>
   <restUrl>${BASE_URL}${APP_URL}?lat=${VALID_LATITUDE}&amp;lon=${VALID_LONGITUDE}&amp;appid=${API_KEY}</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>0</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <validationSteps>
      <id>22e54d7a-a121-4f1b-9e56-b0948990e9f1</id>
      <name>RIGHT_SCHEMA</name>
      <type>JSON_SCHEMA</type>
      <dataType>FILE</dataType>
      <target>RESPONSE</target>
      <data>JSONSchemas/air_pollution_schema.json</data>
      <activate>true</activate>
   </validationSteps>
   <variables>
      <defaultValue>GlobalVariable.API_KEY</defaultValue>
      <description></description>
      <id>81ca6272-5225-45c2-afa5-e03d7853af6a</id>
      <masked>false</masked>
      <name>API_KEY</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.BASE_URL</defaultValue>
      <description></description>
      <id>a3cc3973-2923-4ca4-b3fa-98427306b224</id>
      <masked>false</masked>
      <name>BASE_URL</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.AIR_POLLUTION_URL</defaultValue>
      <description></description>
      <id>2cb7d31e-f51d-4fe7-950b-562c0e40ddce</id>
      <masked>false</masked>
      <name>APP_URL</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.VALID_LATITUDE</defaultValue>
      <description></description>
      <id>503f25ff-9161-4c24-949a-1a401aa8e4ec</id>
      <masked>false</masked>
      <name>VALID_LATITUDE</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.VALID_LONGITUDE</defaultValue>
      <description></description>
      <id>a6cd19d5-fbab-45e5-95c9-021f1cdf01a1</id>
      <masked>false</masked>
      <name>VALID_LONGITUDE</name>
   </variables>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
