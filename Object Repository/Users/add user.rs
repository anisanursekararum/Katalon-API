<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>add user</name>
   <tag></tag>
   <elementGuidId>e095e4b0-6117-4574-9a6b-e3368626d810</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <authorizationRequest>
      <authorizationInfo>
         <entry>
            <key>bearerToken</key>
            <value>${GlobalVariable.accessToken}</value>
         </entry>
      </authorizationInfo>
      <authorizationType>Bearer</authorizationType>
   </authorizationRequest>
   <autoUpdateContent>false</autoUpdateContent>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n    \&quot;name\&quot;: \&quot;${name}\&quot;,\n    \&quot;email\&quot;: \&quot;${email}\&quot;,\n    \&quot;password\&quot;: \&quot;${password}\&quot;\n}&quot;,
  &quot;contentType&quot;: &quot;application/json&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;
}</httpBodyContent>
   <httpBodyType>text</httpBodyType>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>application/json</value>
      <webElementGuid>9e90fc9d-64b7-49e2-9470-d9681a67f084</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Authorization</name>
      <type>Main</type>
      <value>Bearer ${GlobalVariable.accessToken}</value>
      <webElementGuid>448bd84b-8791-4bdf-aef7-8eb29f97f47f</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>8.6.0</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>${GlobalVariable.baseUrl}/users</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>-1</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <variables>
      <defaultValue>GlobalVariable.name</defaultValue>
      <description></description>
      <id>220590b9-6991-4a95-b9d3-56d9e69f2e3a</id>
      <masked>false</masked>
      <name>name</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.email</defaultValue>
      <description></description>
      <id>b7506c93-b6df-4100-a25d-31268dd64972</id>
      <masked>false</masked>
      <name>email</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.password</defaultValue>
      <description></description>
      <id>bd4cad53-3c02-4188-a322-6b0cf3cb4d57</id>
      <masked>false</masked>
      <name>password</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.accessToken</defaultValue>
      <description></description>
      <id>1a66c02e-aebd-463a-9eb4-f6363ec4edac</id>
      <masked>false</masked>
      <name>token</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.userId</defaultValue>
      <description></description>
      <id>5aa30e31-0b95-4954-bc62-695085218637</id>
      <masked>false</masked>
      <name>userId</name>
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
